use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]  // Add Clone here
pub struct Room {
    pub id: i32,
    pub name: String,
    pub room_type: String,
    pub password_hash: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomType {
    Public,
    Private,
    Protected,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
}