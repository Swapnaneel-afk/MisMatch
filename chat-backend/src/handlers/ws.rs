use crate::models::session::{ChatSession, WsMessage};
use crate::utils::avatar::generate_avatar_url;
use actix::{Actor, Handler, StreamHandler, AsyncContext, ActorContext};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use std::sync::Arc;  // Add this import
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    Typing,
    StopTyping,
    UserList  // Add this variant
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
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut connections = self.addr.lock().unwrap();
        
        // Get current user list before adding new user
        let current_users: Vec<String> = connections.iter()
            .map(|(username, _)| username.clone())
            .collect();

        // Add new user
        connections.push((self.username.clone(), ctx.address()));

        // Send current user list to the new user
        let user_list_message = ChatMessage {
            message_type: MessageType::UserList,
            user: "system".to_string(),
            text: "Current users".to_string(),
            timestamp: Utc::now(),
            avatar: generate_avatar_url("system"),
            users: Some(current_users),
            room_id: None,  // Add this
        };

        // Send user list only to the new user
        if let Ok(msg) = serde_json::to_string(&user_list_message) {
            ctx.text(msg);
        }

        // Send join notification to all users
        let join_message = ChatMessage {
            message_type: MessageType::Join,
            user: self.username.clone(),
            text: format!("{} joined the chat", self.username),
            timestamp: Utc::now(),
            avatar: generate_avatar_url(&self.username),
            users: None,
            room_id: None,  // Add this
        };

        let msg = serde_json::to_string(&join_message).unwrap();
        for (_, addr) in connections.iter() {
            addr.do_send(WsMessage(msg.clone()));
        }
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let mut connections = self.addr.lock().unwrap();
        connections.retain(|(_, addr)| addr != &ctx.address());

        let leave_message = ChatMessage {
            message_type: MessageType::Leave,
            user: self.username.clone(),
            text: format!("{} left the chat", self.username),
            timestamp: Utc::now(),
            avatar: generate_avatar_url(&self.username),
            users: None,
            room_id: None,  // Add this
        };

        let msg = serde_json::to_string(&leave_message).unwrap();
        for (_, addr) in connections.iter() {
            addr.do_send(WsMessage(msg.clone()));
        }
    }
}

impl Handler<WsMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(mut chat_message) => {
                        chat_message.user = self.username.clone();
                        chat_message.timestamp = Utc::now();
                        chat_message.avatar = generate_avatar_url(&self.username);

                        // Save chat message to database
                        if let MessageType::Chat = chat_message.message_type {
                            if let Some(room_id) = chat_message.room_id {
                                let db_pool = Arc::clone(&self.db_pool);
                                let content = chat_message.text.clone();
                                let sender_id = chat_message.user.clone();
                                
                                // Spawn a new task to handle database operation
                                let fut = async move {
                                    if let Ok(client) = db_pool.get().await {
                                        let _ = client.execute(
                                            "INSERT INTO messages (room_id, sender_id, content) VALUES ($1, $2, $3)",
                                            &[&room_id, &sender_id, &content]
                                        ).await;
                                    }
                                };
                                actix_web::rt::spawn(fut);
                            }
                        }

                        // Broadcast message
                        if let Ok(msg) = serde_json::to_string(&chat_message) {
                            let connections = self.addr.lock().unwrap();
                            
                            match chat_message.message_type {
                                MessageType::Typing | MessageType::StopTyping => {
                                    for (username, addr) in connections.iter() {
                                        if username != &self.username {
                                            addr.do_send(WsMessage(msg.clone()));
                                        }
                                    }
                                },
                                _ => {
                                    for (_, addr) in connections.iter() {
                                        addr.do_send(WsMessage(msg.clone()));
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing message: {}", e);
                        println!("Problematic message text: {}", text);
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Binary(_)) => {
                println!("Binary messages are not supported");
            }
            Ok(ws::Message::Nop) => (),
            Err(e) => {
                eprintln!("Error handling message: {}", e);
                ctx.stop();
            }
        }
    }
}