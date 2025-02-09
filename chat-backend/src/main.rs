mod models;
mod handlers;
mod utils;

use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::handlers::http::chat_route;
use crate::models::session::ChatSession;

use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use actix_web::http::header;

// Add health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let frontend_url = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "https://mismatch-chat-fmqwl77ne-swapnaneel-afks-projects.vercel.app".to_string());

    let connections: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    println!("Starting server...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
            .route("/health", web::get().to(health_check))  // Add health check route
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}