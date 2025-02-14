use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::db::models::Room;  // Add this import

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    Typing,
    StopTyping,
    UserList,
    NewRoom,
    RoomList
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub user: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub avatar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Room>,
}