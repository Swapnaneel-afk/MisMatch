use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    Typing,
    StopTyping,
    UserList,  // List of users
    RoomList,  // List of available rooms
    JoinRoom,  // Request to join a room
    LeaveRoom, // Request to leave a room
    CreateRoom, // Request to create a new room
    RoomJoined, // Notification that user joined a room
    RoomLeft,   // Notification that user left a room
    Error       // Error message
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub user: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    #[serde(default = "default_avatar")]
    pub avatar: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,  // For sending user list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i32>,  // Room ID for room-specific messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<RoomInfo>>,  // For sending room list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,  // Error message details
}

fn default_avatar() -> String {
    "https://ui-avatars.com/api/?name=anonymous&background=random".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoomInfo {
    pub id: i32,
    pub name: String,
    pub room_type: String,
    pub member_count: i32,
    pub is_protected: bool,
}

// Command messages for room operations
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateRoomCommand {
    pub name: String,
    pub room_type: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JoinRoomCommand {
    pub room_id: i32,
    pub password: Option<String>,
}