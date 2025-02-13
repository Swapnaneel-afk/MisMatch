use actix::Message;
use std::sync::{Arc, Mutex};
use deadpool_postgres::Pool;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

pub struct ChatSession {
    pub id: u32,
    pub username: String,
    pub addr: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>,
    pub db_pool: Arc<Pool>,
}