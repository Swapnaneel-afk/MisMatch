mod models;
mod handlers;
mod utils;

use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};
use crate::handlers::http::chat_route;
use crate::models::session::ChatSession;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let connections: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}