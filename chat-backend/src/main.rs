mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger, rt::time};
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
    
    let mut pool_option = None;
    let mut retry_count = 0;
    const MAX_RETRIES: u8 = 3;
    
    while retry_count < MAX_RETRIES {
        match db::create_pool().await {
            Ok(pool) => {
                println!("Database connected successfully");
                
                // Try to initialize schema
                match pool.get().await {
                    Ok(client) => {
                        match db::schema::create_tables(&client).await {
                            Ok(_) => {
                                println!("Database schema initialized successfully");
                                pool_option = Some(pool);
                                break;
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to create database tables: {}", e);
                                // Try again or continue without schema
                                if e.to_string().contains("syntax error") {
                                    eprintln!("SQL syntax error detected in schema creation - please check your SQL statements");
                                }
                                pool_option = Some(pool);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not get database client for schema initialization: {}", e);
                        if retry_count + 1 < MAX_RETRIES {
                            println!("Retrying database connection in 2 seconds...");
                            time::sleep(Duration::from_secs(2)).await;
                            retry_count += 1;
                            continue;
                        }
                        // Continue despite client error
                        pool_option = Some(pool);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to connect to database: {}", e);
                if retry_count + 1 < MAX_RETRIES {
                    println!("Retrying database connection in 2 seconds...");
                    time::sleep(Duration::from_secs(2)).await;
                    retry_count += 1;
                    continue;
                }
                // Skip retries if we've reached max
                eprintln!("Maximum retries reached. Will operate without database connection.");
                break;
            }
        }
    }

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
            origin_str.contains("vercel.app") ||   // Add this line to allow Vercel domains
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
        if let Some(pool) = &pool_option {
            return app.app_data(web::Data::new(pool.clone()));
        }
        
        app
    })
    .bind(bind_address)?
    .run()
    .await
}