mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse, Error};
use std::sync::{Arc, Mutex};
use crate::handlers::http::chat_route;
use crate::models::session::ChatSession;
use crate::models::message::{ChatMessage, MessageType};
use deadpool_postgres::Pool;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use actix_web::error::ErrorInternalServerError;
use crate::utils::avatar::generate_avatar_url;

use actix_cors::Cors;
use dotenv::dotenv;
use actix_web::http::header;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    #[serde(default)]
    pub id: i32,
    pub name: String,
    pub room_type: String,
    pub password_hash: Option<String>,
    pub created_by: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomRequest {
    pub name: String,
    pub room_type: String,
    pub password: Option<String>,
    pub created_by: String,
}

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub username: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

async fn get_rooms(
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client = db.get().await.map_err(ErrorInternalServerError)?;

    let rows = client.query(
        "SELECT id, name, room_type, password_hash, created_by, created_at FROM rooms ORDER BY created_at DESC",
        &[],
    ).await.map_err(ErrorInternalServerError)?;

    let rooms: Vec<Room> = rows.iter().map(|row| Room {
        id: row.get(0),
        name: row.get(1),
        room_type: row.get(2),
        password_hash: row.get(3),
        created_by: row.get(4),
        created_at: row.get(5),
    }).collect();

    Ok(HttpResponse::Ok().json(rooms))
}


async fn create_room(
    body: web::Json<CreateRoomRequest>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client = db.get().await.map_err(ErrorInternalServerError)?;

    let row = client.query_one(
        "INSERT INTO rooms (name, room_type, password_hash, created_by, created_at) 
         VALUES ($1, $2, $3, $4, $5) 
         RETURNING id, name, room_type, password_hash, created_by, created_at",
        &[
            &body.name,
            &body.room_type,
            &body.password,
            &body.created_by,
            &Utc::now(),
        ],
    ).await.map_err(ErrorInternalServerError)?;

    let room = Room {
        id: row.get(0),
        name: row.get(1),
        room_type: row.get(2),
        password_hash: row.get(3),
        created_by: row.get(4),
        created_at: row.get(5),
    };

    Ok(HttpResponse::Ok().json(room))
}

async fn join_room(
    path: web::Path<i32>,
    body: web::Json<JoinRoomRequest>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let room_id = path.into_inner();
    let client = db.get().await.map_err(ErrorInternalServerError)?;

    client.execute(
        "INSERT INTO room_members (room_id, user_id) 
         VALUES ($1, $2) 
         ON CONFLICT (room_id, user_id) DO NOTHING",
        &[&room_id, &body.username],
    ).await.map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

async fn get_room_messages(
    path: web::Path<i32>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let room_id = path.into_inner();
    let client = db.get().await.map_err(ErrorInternalServerError)?;

    let rows = client.query(
        "SELECT sender_id, content, created_at FROM messages 
         WHERE room_id = $1 
         ORDER BY created_at ASC",
        &[&room_id],
    ).await.map_err(ErrorInternalServerError)?;

    let messages: Vec<ChatMessage> = rows.iter().map(|row| ChatMessage {
        message_type: MessageType::Chat,
        user: row.get(0),
        text: row.get(1),
        timestamp: row.get(2),
        room_id: Some(room_id),
        avatar: format!("https://ui-avatars.com/api/?name={}&background=random", 
            urlencoding::encode(&row.get::<_, String>(0))),
        users: None,
    }).collect();

    Ok(HttpResponse::Ok().json(messages))
}

async fn load_chat_history(
    room_id: i32,
    limit: i64,
    db_pool: &deadpool_postgres::Pool,
) -> Result<Vec<ChatMessage>, Box<dyn std::error::Error>> {
    let client = db_pool.get().await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let rows = client.query(
        "SELECT sender_id, content, created_at 
         FROM messages 
         WHERE room_id = $1 
         ORDER BY created_at DESC 
         LIMIT $2",
        &[&room_id, &limit]
    ).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let messages = rows.iter().map(|row| ChatMessage {
        message_type: MessageType::Chat,
        user: row.get(0),
        text: row.get(1),
        timestamp: row.get(2),
        avatar: generate_avatar_url(&row.get::<_, String>(0)),
        users: None,
        room_id: Some(room_id),
    }).collect();

    Ok(messages)
}

async fn get_room_history(
    path: web::Path<(i32, i64)>,
    db: web::Data<deadpool_postgres::Pool>,
) -> Result<HttpResponse, Error> {
    let (room_id, limit) = path.into_inner();
    
    match load_chat_history(room_id, limit, &db).await {
        Ok(messages) => Ok(HttpResponse::Ok().json(messages)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = db::create_pool().await;

    let client = pool.get().await.expect("Failed to get DB client");
    db::schema::create_tables(&client).await.expect("Failed to create tables");

    let connections: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(connections.clone()))
            .wrap(cors)
            .route("/ws", web::get().to(chat_route))
            .route("/health", web::get().to(health_check))
            .route("/api/rooms", web::get().to(get_rooms))
            .route("/api/rooms", web::post().to(create_room))
            .route("/api/rooms/{id}/join", web::post().to(join_room))
            .route("/api/rooms/{id}/messages", web::get().to(get_room_messages))
            .wrap(actix_web::middleware::Logger::default())
            .route("/api/rooms/{id}/history/{limit}", web::get().to(get_room_history))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}