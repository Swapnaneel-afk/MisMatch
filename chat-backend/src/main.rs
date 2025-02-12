mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Arc, Mutex};
use crate::handlers::http::chat_route;
use crate::models::session::ChatSession;

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
            .allowed_origin("http://localhost:3000")  // Changed for local development
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE
            ])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
            .route("/health", web::get().to(health_check))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?  // Local development address
    .run()
    .await
}