use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::Rng;
use std::sync::{Arc, Mutex};
use crate::models::session::ChatSession;
use deadpool_postgres::Pool;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>>,
    db_pool: Option<web::Data<Pool>>,
) -> Result<HttpResponse, Error> {
    let username = req.query_string()
        .split('=')
        .nth(1)
        .unwrap_or("Anonymous")
        .to_string();

    let username = urlencoding::decode(&username)
        .unwrap_or_else(|_| "Anonymous".into())
        .into_owned();

    println!("New connection from user: {}", username);

    // Create a ChatSession with database pool if available
    let resp = ws::start(
        ChatSession {
            id: rand::thread_rng().gen_range(1..=1000),
            username,
            user_id: None, // Will be set when we look up or create user
            current_room_id: None,
            addr: srv.get_ref().clone(),
            db_pool: db_pool.map(|p| p.get_ref().clone()),
        },
        &req,
        stream,
    );
    resp
}

// Endpoint to create a room
pub async fn create_room(
    req: web::Json<crate::models::message::CreateRoomCommand>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let command = req.into_inner();
    let pool = db_pool.get_ref().clone();
    
    // Get user_id from request (would normally use authentication)
    let user_id = 1; // Default for testing
    
    match pool.get().await {
        Ok(client) => {
            match crate::db::create_room(
                &client,
                &command.name,
                &command.room_type,
                command.password.as_deref(),
                user_id,
            ).await {
                Ok(room_id) => {
                    Ok(HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "room_id": room_id
                    })))
                },
                Err(e) => {
                    eprintln!("Error creating room: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": format!("Database error: {}", e)
                    })))
                }
            }
        },
        Err(e) => {
            eprintln!("Error getting database client: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "Database connection error"
            })))
        }
    }
}

// Endpoint to list all rooms
pub async fn list_rooms(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let pool = db_pool.get_ref().clone();
    
    match pool.get().await {
        Ok(client) => {
            match crate::db::get_rooms(&client).await {
                Ok(rooms) => {
                    // Convert to RoomInfo objects
                    let room_infos: Vec<crate::models::message::RoomInfo> = rooms.iter()
                        .map(|room| {
                            crate::models::message::RoomInfo {
                                id: room.id,
                                name: room.name.clone(),
                                room_type: format!("{:?}", room.room_type).to_lowercase(),
                                member_count: 0, // We would need another query to get this
                                is_protected: room.password_hash.is_some(),
                            }
                        })
                        .collect();
                    
                    Ok(HttpResponse::Ok().json(room_infos))
                },
                Err(e) => {
                    eprintln!("Error getting rooms: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": format!("Database error: {}", e)
                    })))
                }
            }
        },
        Err(e) => {
            eprintln!("Error getting database client: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "Database connection error"
            })))
        }
    }
}