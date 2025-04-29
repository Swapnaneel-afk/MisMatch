use actix::Message;
use std::sync::{Arc, Mutex};
use deadpool_postgres::Pool;
use std::collections::HashMap;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

pub struct ChatSession {
    pub id: u32,
    pub username: String,
    pub user_id: Option<i32>,  // Database user ID if authenticated
    pub current_room_id: Option<i32>,  // Current room the user is in
    pub addr: Arc<Mutex<Vec<(String, actix::Addr<ChatSession>)>>>,
    pub db_pool: Option<Pool>, // Database connection pool
}

// Room state to manage room-specific connections
pub struct RoomState {
    // Map of room_id to a list of (username, address) pairs
    pub rooms: HashMap<i32, Vec<(String, actix::Addr<ChatSession>)>>,
}

impl RoomState {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    pub fn add_user_to_room(&mut self, room_id: i32, username: String, addr: actix::Addr<ChatSession>) {
        self.rooms.entry(room_id)
            .or_insert_with(Vec::new)
            .push((username, addr));
    }

    pub fn remove_user_from_room(&mut self, room_id: i32, addr: &actix::Addr<ChatSession>) {
        if let Some(users) = self.rooms.get_mut(&room_id) {
            users.retain(|(_, user_addr)| user_addr != addr);
            
            // If room is empty, remove it
            if users.is_empty() {
                self.rooms.remove(&room_id);
            }
        }
    }

    pub fn get_room_members(&self, room_id: i32) -> Vec<String> {
        self.rooms.get(&room_id)
            .map(|users| users.iter().map(|(username, _)| username.clone()).collect())
            .unwrap_or_default()
    }

    pub fn broadcast_to_room(&self, room_id: i32, message: WsMessage, except_username: Option<&str>) {
        if let Some(users) = self.rooms.get(&room_id) {
            for (username, addr) in users {
                if except_username.is_none() || except_username.unwrap() != username {
                    addr.do_send(message.clone());
                }
            }
        }
    }
}