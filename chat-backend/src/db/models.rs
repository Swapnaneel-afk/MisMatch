use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use tokio_postgres::{Client, Error};

// User Model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

// Room Model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    pub id: Option<i32>,
    pub name: String,
    pub room_type: String,
    pub password_hash: Option<String>,
    pub created_by: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

// Message Model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Option<i32>,
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

// Database operations for users
impl User {
    pub async fn create(client: &Client, user: &User) -> Result<User, Error> {
        let row = client
            .query_one(
                "INSERT INTO users (username, password_hash, avatar_url) 
                 VALUES ($1, $2, $3) 
                 RETURNING id, username, password_hash, avatar_url, created_at",
                &[
                    &user.username,
                    &user.password_hash,
                    &user.avatar_url,
                ],
            )
            .await?;
        
        Ok(User {
            id: Some(row.get(0)),
            username: row.get(1),
            password_hash: row.get(2),
            avatar_url: row.get(3),
            created_at: row.get(4),
        })
    }
    
    pub async fn find_by_username(client: &Client, username: &str) -> Result<Option<User>, Error> {
        let result = client
            .query_opt(
                "SELECT id, username, password_hash, avatar_url, created_at 
                 FROM users WHERE username = $1",
                &[&username],
            )
            .await?;
        
        if let Some(row) = result {
            Ok(Some(User {
                id: Some(row.get(0)),
                username: row.get(1),
                password_hash: row.get(2),
                avatar_url: row.get(3),
                created_at: row.get(4),
            }))
        } else {
            Ok(None)
        }
    }
}

// Database operations for rooms
impl Room {
    pub async fn create(client: &Client, room: &Room) -> Result<Room, Error> {
        let row = client
            .query_one(
                "INSERT INTO rooms (name, type, password_hash, created_by) 
                 VALUES ($1, $2, $3, $4) 
                 RETURNING id, name, type, password_hash, created_by, created_at",
                &[
                    &room.name,
                    &room.room_type,
                    &room.password_hash,
                    &room.created_by,
                ],
            )
            .await?;
        
        Ok(Room {
            id: Some(row.get(0)),
            name: row.get(1),
            room_type: row.get(2),
            password_hash: row.get(3),
            created_by: row.get(4),
            created_at: row.get(5),
        })
    }
    
    pub async fn find_all(client: &Client) -> Result<Vec<Room>, Error> {
        let rows = client
            .query(
                "SELECT id, name, type, password_hash, created_by, created_at 
                 FROM rooms",
                &[],
            )
            .await?;
        
        let rooms = rows
            .into_iter()
            .map(|row| Room {
                id: Some(row.get(0)),
                name: row.get(1),
                room_type: row.get(2),
                password_hash: row.get(3),
                created_by: row.get(4),
                created_at: row.get(5),
            })
            .collect();
        
        Ok(rooms)
    }
    
    pub async fn join_room(
        client: &Client, 
        user_id: i32, 
        room_id: i32, 
        role: &str
    ) -> Result<(), Error> {
        client
            .execute(
                "INSERT INTO room_members (room_id, user_id, role) 
                 VALUES ($1, $2, $3)
                 ON CONFLICT (room_id, user_id) DO NOTHING",
                &[&room_id, &user_id, &role],
            )
            .await?;
        
        Ok(())
    }
}

// Database operations for messages
impl Message {
    pub async fn create(client: &Client, message: &Message) -> Result<Message, Error> {
        let row = client
            .query_one(
                "INSERT INTO messages (room_id, sender_id, content) 
                 VALUES ($1, $2, $3) 
                 RETURNING id, room_id, sender_id, content, created_at",
                &[
                    &message.room_id,
                    &message.sender_id,
                    &message.content,
                ],
            )
            .await?;
        
        Ok(Message {
            id: Some(row.get(0)),
            room_id: row.get(1),
            sender_id: row.get(2),
            content: row.get(3),
            created_at: row.get(4),
        })
    }
    
    pub async fn find_by_room(client: &Client, room_id: i32, limit: i64) -> Result<Vec<Message>, Error> {
        let rows = client
            .query(
                "SELECT m.id, m.room_id, m.sender_id, m.content, m.created_at 
                 FROM messages m
                 WHERE m.room_id = $1
                 ORDER BY m.created_at DESC
                 LIMIT $2",
                &[&room_id, &limit],
            )
            .await?;
        
        let messages = rows
            .into_iter()
            .map(|row| Message {
                id: Some(row.get(0)),
                room_id: row.get(1),
                sender_id: row.get(2),
                content: row.get(3),
                created_at: row.get(4),
            })
            .collect();
        
        Ok(messages)
    }
}