use actix::{Actor, Handler, Message, StreamHandler, AsyncContext, ActorContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use rand::Rng;
use chrono::{DateTime, Utc};

// Single definition of MessageType with correct attributes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Chat,
    Join,
    Leave,
    Typing,
    StopTyping
}

// Single definition of ChatMessage with all needed attributes
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

#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

struct ChatSession {
    id: u32,
    username: String,
    addr: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>,
}

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
                        
                        #[derive(Deserialize)]
                        struct PartialMessage {
                            message_type: MessageType,
                            user: String,
                            text: String,
                        }

                        if let Ok(partial) = serde_json::from_str::<PartialMessage>(&text) {
                            let complete_message = ChatMessage {
                                message_type: partial.message_type,
                                user: self.username.clone(),
                                text: partial.text,
                                timestamp: Utc::now(),
                                avatar: generate_avatar_url(&self.username),
                            };

                            if let Ok(msg) = serde_json::to_string(&complete_message) {
                                for (_, addr) in self.addr.lock().unwrap().iter() {
                                    addr.do_send(WsMessage(msg.clone()));
                                }
                            }
                        }
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

fn generate_avatar_url(username: &str) -> String {
    format!("https://ui-avatars.com/api/?name={}&background=random", 
        urlencoding::encode(username))
}

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>>,
) -> Result<HttpResponse, Error> {
    let username = req.query_string()
        .split('=')
        .nth(1)
        .unwrap_or("Anonymous")
        .to_string();

    let username = urlencoding::decode(&username)
        .unwrap_or_else(|_| "Anonymous".into())
        .into_owned();

    println!("New connection from user: {}", username);

    let resp = ws::start(
        ChatSession {
            id: rand::thread_rng().gen_range(1..=1000),
            username,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let connections: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>> = 
        Arc::new(Mutex::new(Vec::new()));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}