mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
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
    // Load environment variables and set up logging
    dotenv().ok();
    env_logger::init();
    
    println!("Starting MisMatch backend server...");

    // Create database pool (optional)
    println!("Attempting to connect to database...");
    let pool_result = db::create_pool().await;
    
    let pool = if let Ok(pool) = pool_result {
        println!("Database connected successfully");
        // Try to initialize schema
        if let Ok(client) = pool.get().await {
            if let Err(e) = db::schema::create_tables(&client).await {
                eprintln!("Warning: Failed to create database tables: {}", e);
                // Continue despite schema error
            }
        } else {
            eprintln!("Warning: Could not get database client for schema initialization");
            // Continue despite client error
        }
        Some(pool)
    } else {
        eprintln!("Warning: Failed to connect to database. Will operate without database.");
        None
    };

    // Create shared state for WebSocket connections
    let connections: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    // Get port from environment variables (for Docker/Railway compatibility)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    println!("Starting server at http://{}", bind_address);

    // Create and start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                // Allow all origins in development mode
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.starts_with("http://localhost") || 
                origin_str.contains("railway.app") ||
                origin_str.starts_with("https://") 
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE
            ])
            .supports_credentials();

        let app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
            .route("/health", web::get().to(health_check));
            
        // Add database pool to app data only if it's available
        if let Some(pool) = &pool {
            return app.app_data(web::Data::new(pool.clone()));
        }
        
        app
    })
    .bind(bind_address)?
    .run()
    .await
}