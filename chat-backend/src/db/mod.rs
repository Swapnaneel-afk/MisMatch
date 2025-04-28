pub mod schema;
pub mod models;

use deadpool_postgres::{Config, Pool, Runtime, CreatePoolError};
use tokio_postgres::NoTls;
use std::env;

pub async fn create_pool() -> Result<Pool, CreatePoolError> {
    // Hardcoded connection string for deployment
    let database_url = "postgresql://postgres:zVotVSHvjVqNmTVXgesgBgrAqtAgTbSD@hopper.proxy.rlwy.net:13931/railway";
    
    // Parse the connection string
    if let Ok(config) = deadpool_postgres::config::ConfigParser::parse_url(database_url) {
        // Use the parsed configuration
        return config.create_pool(Some(Runtime::Tokio1), NoTls);
    }
    
    // Fallback to individual environment variables (should not reach here)
    let mut cfg = Config::new();
    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));
    cfg.port = Some(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap());
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or_else(|_| "chat_db".to_string()));
    cfg.user = Some(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string()));

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}