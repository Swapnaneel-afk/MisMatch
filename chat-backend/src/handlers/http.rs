use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::Rng;
use std::sync::{Arc, Mutex};
use crate::models::session::ChatSession;

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>>,
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

    let resp = ws::start(
        ChatSession {
            id: rand::thread_rng().gen_range(1..=1000),
            username,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}