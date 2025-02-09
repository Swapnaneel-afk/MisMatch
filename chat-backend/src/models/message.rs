use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    Typing,
    StopTyping
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub user: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    #[serde(default = "default_avatar")]
    pub avatar: String,
}

fn default_avatar() -> String {
    "https://ui-avatars.com/api/?name=anonymous&background=random".to_string()
}