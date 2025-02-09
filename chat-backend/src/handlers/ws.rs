use crate::models::message::{ChatMessage, MessageType};
use crate::models::session::{ChatSession, WsMessage};
use crate::utils::avatar::generate_avatar_url;
use actix::{Actor, Handler, StreamHandler, AsyncContext, ActorContext}; // Added missing traits
use actix_web_actors::ws;
use chrono::Utc;

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut connections = self.addr.lock().unwrap();
        connections.push((self.username.clone(), ctx.address()));

        let join_message = ChatMessage {
            message_type: MessageType::Join,
            user: self.username.clone(),
            text: format!("{} joined the chat", self.username),
            timestamp: Utc::now(),
            avatar: generate_avatar_url(&self.username),
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
        println!("Handling WsMessage: {}", msg.0);
        ctx.text(msg.0); // Removed generic parameter
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
                            println!("Broadcasting message: {}", msg);
                            let connections = self.addr.lock().unwrap();
                            println!("Number of connections: {}", connections.len());
                            
                            for (username, addr) in connections.iter() {
                                println!("Sending to user: {}", username);
                                addr.do_send(WsMessage(msg.clone()));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error parsing message: {}", e);
                        println!("Problematic message text: {}", text);
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg), // Removed generic parameter
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