use actix::{Actor, Handler, Message, StreamHandler, AsyncContext, ActorContext};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use rand::Rng;

// Message structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub text: String,
}

// Custom message type for our actor
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(String);

// WebSocket actor
struct ChatSession {
    id: u32,  // Changed to u32 for simpler random generation
    addr: Arc<Mutex<Vec<actix::Addr<ChatSession>>>>,
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Store the address when the session starts
        self.addr.lock().unwrap().push(ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // Remove the address when the session stops
        let mut addresses = self.addr.lock().unwrap();
        addresses.retain(|addr| addr != &ctx.address());
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
                // Broadcast message to all clients
                let msg = text.to_string();
                for addr in self.addr.lock().unwrap().iter() {
                    addr.do_send(WsMessage(msg.clone()));
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Arc<Mutex<Vec<actix::Addr<ChatSession>>>>>,
) -> Result<HttpResponse, Error> {
    let id = rand::thread_rng().gen_range(1..=1000);  // Generate random ID between 1 and 1000
    let resp = ws::start(
        ChatSession {
            id,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Shared state for active connections
    let connections: Arc<Mutex<Vec<actix::Addr<ChatSession>>>> = Arc::new(Mutex::new(Vec::new()));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(connections.clone()))
            .route("/ws", web::get().to(chat_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}