pub mod schema;
pub mod models;

use deadpool_postgres::{Config, Pool, Runtime, CreatePoolError, Client};
use tokio_postgres::NoTls;
use std::env;
use argon2::{self, Argon2, password_hash::{PasswordHasher, SaltString, PasswordVerifier}};
use argon2::password_hash::rand_core::OsRng;
use rand::Rng;
use crate::db::models::{Room, RoomType, Message};
use chrono::{DateTime, Utc};
use tokio_postgres::types::Type;

pub async fn create_pool() -> Result<Pool, CreatePoolError> {
    // Check if DATABASE_URL is provided
    if let Ok(database_url) = env::var("DATABASE_URL") {
        // Parse the connection string manually
        println!("Using DATABASE_URL for connection");
        
        // Extract connection details from URL format: postgresql://username:password@host:port/dbname
        let url = database_url.trim_start_matches("postgresql://");
        
        // Split credentials and host parts
        if let Some(creds_host_split) = url.split_once('@') {
            let (creds, host_part) = creds_host_split;
            
            // Split username and password
            if let Some(user_pass_split) = creds.split_once(':') {
                let (username, password) = user_pass_split;
                
                // Split host:port and database name
                if let Some(host_db_split) = host_part.split_once('/') {
                    let (host_port, dbname) = host_db_split;
                    
                    // Split host and port
                    let (host, port) = if let Some(host_port_split) = host_port.split_once(':') {
                        host_port_split
                    } else {
                        (host_port, "5432")
                    };
                    
                    let mut cfg = Config::new();
                    cfg.host = Some(host.to_string());
                    cfg.port = Some(port.parse().unwrap_or(5432));
                    cfg.dbname = Some(dbname.to_string());
                    cfg.user = Some(username.to_string());
                    cfg.password = Some(password.to_string());
                    
                    return cfg.create_pool(Some(Runtime::Tokio1), NoTls);
                }
            }
        }
        println!("Failed to parse DATABASE_URL, falling back to individual variables");
    }
    
    // Fallback to individual environment variables
    let mut cfg = Config::new();
    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));
    cfg.port = Some(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap());
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or_else(|_| "chat_db".to_string()));
    cfg.user = Some(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string()));

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

// User functions
pub async fn create_user(
    client: &Client, 
    username: &str, 
    password: &str
) -> Result<i32, tokio_postgres::Error> {
    // Hash password
    let password_hash = hash_password(password);
    
    let row = client.query_one(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id",
        &[&username, &password_hash],
    ).await?;
    
    Ok(row.get(0))
}

pub async fn authenticate_user(
    client: &Client,
    username: &str,
    password: &str
) -> Result<Option<i32>, tokio_postgres::Error> {
    let rows = client.query(
        "SELECT id, password_hash FROM users WHERE username = $1",
        &[&username],
    ).await?;
    
    if let Some(row) = rows.first() {
        let password_hash: String = row.get(1);
        if verify_password(password, &password_hash) {
            let user_id: i32 = row.get(0);
            return Ok(Some(user_id));
        }
    }
    
    Ok(None)
}

pub async fn get_user_by_username(
    client: &Client,
    username: &str
) -> Result<Option<(i32, String)>, tokio_postgres::Error> {
    let rows = client.query(
        "SELECT id, username FROM users WHERE username = $1",
        &[&username],
    ).await?;
    
    if let Some(row) = rows.first() {
        let user_id: i32 = row.get(0);
        let username: String = row.get(1);
        return Ok(Some((user_id, username)));
    }
    
    Ok(None)
}

// Room functions
pub async fn create_room(
    client: &Client,
    name: &str,
    room_type: &str,
    password: Option<&str>,
    created_by: i32
) -> Result<i32, tokio_postgres::Error> {
    // Hash password if provided
    let password_hash = password.map(|p| hash_password(p));
    
    let row = client.query_one(
        "INSERT INTO rooms (name, type, password_hash, created_by) VALUES ($1, $2, $3, $4) RETURNING id",
        &[&name, &room_type, &password_hash, &created_by],
    ).await?;
    
    let room_id: i32 = row.get(0);
    
    // Add the creator as an admin
    client.execute(
        "INSERT INTO room_members (room_id, user_id, role) VALUES ($1, $2, 'admin')",
        &[&room_id, &created_by],
    ).await?;
    
    Ok(room_id)
}

pub async fn get_rooms(client: &Client) -> Result<Vec<Room>, tokio_postgres::Error> {
    let rows = client.query(
        "SELECT r.id, r.name, r.type, r.password_hash, r.created_by, r.created_at::text 
         FROM rooms r",
        &[],
    ).await?;
    
    let mut rooms = Vec::new();
    for row in rows {
        let room_type_str: String = row.get(2);
        let room_type = match room_type_str.as_str() {
            "public" => RoomType::Public,
            "private" => RoomType::Private,
            "protected" => RoomType::Protected,
            _ => RoomType::Public,
        };
        
        // Parse the timestamp string
        let timestamp_str: String = row.get(5);
        let created_at = DateTime::parse_from_rfc3339(&timestamp_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        rooms.push(Room {
            id: row.get(0),
            name: row.get(1),
            room_type,
            password_hash: row.get(3),
            created_by: row.get(4),
            created_at,
        });
    }
    
    Ok(rooms)
}

pub async fn join_room(
    client: &Client,
    room_id: i32,
    user_id: i32
) -> Result<bool, tokio_postgres::Error> {
    // Check if user is already a member
    let existing = client.query_one(
        "SELECT COUNT(*) FROM room_members WHERE room_id = $1 AND user_id = $2",
        &[&room_id, &user_id],
    ).await?;
    
    let count: i64 = existing.get(0);
    if count > 0 {
        return Ok(true); // Already joined
    }
    
    // Join the room as a member
    client.execute(
        "INSERT INTO room_members (room_id, user_id, role) VALUES ($1, $2, 'member')",
        &[&room_id, &user_id],
    ).await?;
    
    Ok(true)
}

// Message functions
pub async fn save_message(
    client: &Client,
    room_id: i32,
    sender_id: i32,
    content: &str
) -> Result<i32, tokio_postgres::Error> {
    let row = client.query_one(
        "INSERT INTO messages (room_id, sender_id, content) VALUES ($1, $2, $3) RETURNING id",
        &[&room_id, &sender_id, &content],
    ).await?;
    
    Ok(row.get(0))
}

pub async fn get_room_messages(
    client: &Client,
    room_id: i32,
    limit: i64
) -> Result<Vec<Message>, tokio_postgres::Error> {
    let rows = client.query(
        "SELECT id, room_id, sender_id, content, created_at::text 
         FROM messages 
         WHERE room_id = $1 
         ORDER BY created_at DESC 
         LIMIT $2",
        &[&room_id, &limit],
    ).await?;
    
    let mut messages = Vec::new();
    for row in rows {
        // Parse the timestamp string
        let timestamp_str: String = row.get(4);
        let created_at = DateTime::parse_from_rfc3339(&timestamp_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        messages.push(Message {
            id: row.get(0),
            room_id: row.get(1),
            sender_id: row.get(2),
            content: row.get(3),
            created_at,
        });
    }
    
    // Reverse to get oldest first
    messages.reverse();
    
    Ok(messages)
}

// Helper functions for password hashing
fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .unwrap_or_else(|_| String::from(""))
}

fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match argon2::password_hash::PasswordHash::new(hash) {
        Ok(parsed) => parsed,
        Err(_) => return false,
    };
    
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}