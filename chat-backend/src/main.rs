mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger, rt::time};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::handlers::http::{chat_route, create_room, list_rooms};
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
            .allow_any_origin() // Allow all origins for testing
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::ORIGIN,
                header::ACCESS_CONTROL_REQUEST_METHOD,
                header::ACCESS_CONTROL_REQUEST_HEADERS,
            ])
            .max_age(3600)
            .supports_credentials();
            
        // Build the app with core routes first
        let app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(connections.clone()))
            .route("/health", web::get().to(health_check));
            
        // Add database pool to app data only if it's available
        if let Some(ref pool) = pool_option {
            let db_pool = web::Data::new(pool.clone());
            
            // Routes that need database access
            return app
                .app_data(db_pool.clone())
                // WebSocket route with optional DB pool
                .route("/ws", web::get().to(move |req, stream, srv| {
                    chat_route(req, stream, srv, Some(db_pool.clone()))
                }))
                // Room management routes
                .route("/api/rooms", web::get().to(list_rooms))
                .route("/api/rooms", web::post().to(create_room));
        }
        
        // Default route without DB for WebSockets
        app.route("/ws", web::get().to(move |req, stream, srv| {
            chat_route(req, stream, srv, None::<web::Data<deadpool_postgres::Pool>>)
        }))
    })
    .bind(bind_address)?
    .run()
    .await
}