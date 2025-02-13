pub mod schema;

use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;
use std::env;

pub async fn create_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));
    cfg.port = Some(env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap());
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or_else(|_| "chat_db".to_string()));
    cfg.user = Some(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or_else(|_| "your_password".to_string()));

    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("Failed to create pool")
}