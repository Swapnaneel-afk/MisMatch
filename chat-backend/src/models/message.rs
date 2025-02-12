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
    UserList  // New message type
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
}

fn default_avatar() -> String {
    "https://ui-avatars.com/api/?name=anonymous&background=random".to_string()
}