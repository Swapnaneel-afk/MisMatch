use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::Rng;
use std::sync::{Arc, Mutex};
use crate::models::session::ChatSession;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Vec<(String, i32, actix::Addr<ChatSession>)>>>>,
) -> Result<HttpResponse, Error> {
    // Parse username from query string
    let username = req.query_string()
        .split('&')
        .find(|s| s.starts_with("username="))
        .and_then(|s| s.split('=').nth(1))
        .unwrap_or("Anonymous")
        .to_string();

    let username = urlencoding::decode(&username)
        .unwrap_or_else(|_| "Anonymous".into())
        .into_owned();
    
    // Parse room ID from query string
    let room_id = req.query_string()
        .split('&')
        .find(|s| s.starts_with("roomId="))
        .and_then(|s| s.split('=').nth(1))
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0); // Default to room 0 if not specified

    println!("New connection from user: {} to room: {}", username, room_id);

    let resp = ws::start(
        ChatSession {
            id: rand::thread_rng().gen_range(1..=1000),
            username,
            room_id,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}