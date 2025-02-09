use crate::models::message::{ChatMessage, MessageType};
use crate::models::session::{ChatSession, WsMessage};
use crate::utils::avatar::generate_avatar_url;
use actix::{Actor, Handler, StreamHandler, AsyncContext, ActorContext};
use actix_web_actors::ws;
use chrono::Utc;

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
                println!("Received text message: {}", text);

                match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(mut chat_message) => {
                        chat_message.user = self.username.clone();
                        chat_message.timestamp = Utc::now();
                        chat_message.avatar = generate_avatar_url(&self.username);

                        println!("Processed message: {:?}", chat_message);

                        if let Ok(msg) = serde_json::to_string(&chat_message) {
                            let connections = self.addr.lock().unwrap();
                            
                            match chat_message.message_type {
                                MessageType::Typing | MessageType::StopTyping => {
                                    // Send typing indicators to everyone except the sender
                                    for (username, addr) in connections.iter() {
                                        if username != &self.username {
                                            addr.do_send(WsMessage(msg.clone()));
                                        }
                                    }
                                },
                                _ => {
                                    // Send other messages to everyone
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