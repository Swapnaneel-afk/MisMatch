mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use std::sync::{Arc, Mutex};
use crate::handlers::http::chat_route;
use crate::models::session::ChatSession;
use crate::handlers::api::{create_user, get_rooms, create_room, join_room, get_room_messages};

use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use actix_web::http::header;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Get configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("Creating database pool...");
    let pool = match db::create_pool().await {
        Ok(pool) => {
            println!("Database pool created successfully");
            pool
        },
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Get a client from the pool and create tables if needed
    println!("Initializing database schema...");
    match pool.get().await {
        Ok(client) => {
            match db::schema::create_tables(&client).await {
                Ok(_) => println!("Database tables created or verified successfully"),
                Err(e) => {
                    eprintln!("Failed to create database tables: {}", e);
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to get database client: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };

    // Create shared state for WebSocket connections
    let connections: Arc<Mutex<Vec<(String, i32, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    println!("Starting server at http://{}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE
            ])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(connections.clone()))
            .app_data(web::Data::new(pool.clone()))
            .route("/ws", web::get().to(chat_route))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .route("/users", web::post().to(create_user))
                    .route("/rooms", web::get().to(get_rooms))
                    .route("/rooms/join", web::post().to(join_room))
                    .route("/rooms/create/{user_id}", web::post().to(create_room))
                    .route("/rooms/{room_id}/messages", web::get().to(get_room_messages))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}