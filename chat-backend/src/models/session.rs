use actix::{Actor, StreamHandler, Message, Handler, Running, ActorContext, AsyncContext};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use crate::utils::avatar::generate_avatar_url;

// Message types
#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub message_type: String,
    pub user: String,
    pub text: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i32>,
}

pub struct ChatSession {
    pub id: u32,
    pub username: String,
    pub room_id: i32,
    pub addr: Arc<Mutex<Vec<(String, i32, actix::Addr<ChatSession>)>>>,
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Add self to the shared connections list
        let addr = ctx.address();
        self.addr.lock().unwrap().push((self.username.clone(), self.room_id, addr));
        
        // Broadcast user join notification
        let join_msg = WsMessage {
            message_type: "join".to_string(),
            user: self.username.clone(),
            text: format!("{} has joined the chat", self.username),
            timestamp: chrono::Utc::now().to_rfc3339(),
            room_id: Some(self.room_id),
        };
        
        // Broadcast user list to all connected clients in the same room
        self.broadcast_user_list();
        
        // Broadcast the join message to the room
        self.broadcast_message(&join_msg);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // Remove self from connections
        let mut connections = self.addr.lock().unwrap();
        connections.retain(|(username, room_id, _)| !(username == &self.username && room_id == &self.room_id));
        
        // Broadcast leave message
        let leave_msg = WsMessage {
            message_type: "leave".to_string(),
            user: self.username.clone(),
            text: format!("{} has left the chat", self.username),
            timestamp: chrono::Utc::now().to_rfc3339(),
            room_id: Some(self.room_id),
        };
        
        self.broadcast_message(&leave_msg);
        
        // Broadcast updated user list
        self.broadcast_user_list();
        
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Parse the incoming message
                if let Ok(mut ws_message) = serde_json::from_str::<WsMessage>(&text) {
                    // Ensure room_id is set
                    if ws_message.room_id.is_none() {
                        ws_message.room_id = Some(self.room_id);
                    }
                    
                    match ws_message.message_type.as_str() {
                        "chat" => {
                            // Add avatar to the message
                            let avatar = generate_avatar_url(&ws_message.user);
                            
                            // Store message in database (would be implemented in a real app)
                            // For a complete implementation, we would:
                            // 1. Get the user_id from the database based on username
                            // 2. Create a new message in the database
                            // 3. Return the created message with additional data
                            
                            // Broadcast to room
                            self.broadcast_message(&ws_message);
                        }
                        "typing" | "stop_typing" => {
                            // Forward typing indicators to the room
                            self.broadcast_message(&ws_message);
                        }
                        _ => {
                            println!("Unknown message type: {}", ws_message.message_type);
                        }
                    }
                } else {
                    println!("Failed to parse message: {}", text);
                }
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                // Do nothing with pong responses
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {
                ctx.stop();
            }
        }
    }
}

impl Handler<WsMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        // Send a message to this client
        let json = serde_json::to_string(&msg).unwrap();
        ctx.text(json);
    }
}

impl ChatSession {
    // Helper method to broadcast a message to all clients in the same room
    fn broadcast_message(&self, message: &WsMessage) {
        if let Ok(connections) = self.addr.lock() {
            let room_id = message.room_id.unwrap_or(self.room_id);
            
            // Only broadcast to clients in the same room
            for (_, client_room_id, addr) in connections.iter().filter(|(_, r, _)| r == &room_id) {
                addr.do_send(WsMessage {
                    message_type: message.message_type.clone(),
                    user: message.user.clone(),
                    text: message.text.clone(),
                    timestamp: message.timestamp.clone(),
                    room_id: Some(room_id),
                });
            }
        }
    }
    
    // Helper method to broadcast the list of online users in the same room
    fn broadcast_user_list(&self) {
        if let Ok(connections) = self.addr.lock() {
            // Filter users to only those in the same room
            let users: Vec<String> = connections.iter()
                .filter(|(_, room_id, _)| room_id == &self.room_id)
                .map(|(username, _, _)| username.clone())
                .collect();
            
            // Create a user list message
            let user_list_msg = serde_json::json!({
                "message_type": "user_list",
                "users": users,
                "room_id": self.room_id
            });
            
            // Send to all clients in the same room
            for (_, client_room_id, addr) in connections.iter().filter(|(_, r, _)| r == &self.room_id) {
                addr.do_send(WsMessage {
                    message_type: "user_list".to_string(),
                    user: "system".to_string(),
                    text: user_list_msg.to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                    room_id: Some(self.room_id),
                });
            }
        }
    }
}