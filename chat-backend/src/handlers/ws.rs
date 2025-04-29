use crate::models::message::{ChatMessage, MessageType, RoomInfo, CreateRoomCommand, JoinRoomCommand};
use crate::models::session::{ChatSession, WsMessage};
use crate::utils::avatar::generate_avatar_url;
use crate::db;
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

        // Add new user to global connections
        connections.push((self.username.clone(), ctx.address()));

        // Register user in the database if not already registered
        if let Some(ref pool) = self.db_pool {
            // Using a cloned username to avoid borrowing issues
            let username_clone = self.username.clone();
            let session_addr = ctx.address();
            
            // Spawn a future to handle database operations
            let pool_clone = pool.clone();
            actix::spawn(async move {
                match pool_clone.get().await {
                    Ok(client) => {
                        // Check if user exists, or create a new one with a random password
                        match db::get_user_by_username(&client, &username_clone).await {
                            Ok(Some((user_id, _))) => {
                                println!("User {} already exists with ID {}", username_clone, user_id);
                                // Set the user_id in the session
                                session_addr.do_send(WsMessage(format!("{{\"user_id_update\":{}}}", user_id)));
                            },
                            Ok(None) => {
                                // Create a new user with random password for now (this is just for testing)
                                let random_password = format!("pass_{}", rand::random::<u32>());
                                match db::create_user(&client, &username_clone, &random_password).await {
                                    Ok(user_id) => {
                                        println!("Created new user {} with ID {}", username_clone, user_id);
                                        // Set the user_id in the session
                                        session_addr.do_send(WsMessage(format!("{{\"user_id_update\":{}}}", user_id)));
                                    },
                                    Err(e) => {
                                        eprintln!("Error creating user {}: {}", username_clone, e);
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("Database error when checking user {}: {}", username_clone, e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error getting database client: {}", e);
                    }
                }
            });
        }

        // Send current user list to the new user
        let user_list_message = ChatMessage {
            message_type: MessageType::UserList,
            user: "system".to_string(),
            text: "Current users".to_string(),
            timestamp: Utc::now(),
            avatar: generate_avatar_url("system"),
            users: Some(current_users),
            room_id: None,
            rooms: None,
            error: None,
        };

        // Send user list only to the new user
        if let Ok(msg) = serde_json::to_string(&user_list_message) {
            ctx.text(msg);
        }

        // Get available rooms list from database
        if let Some(ref pool) = self.db_pool {
            let pool_clone = pool.clone();
            let addr = ctx.address();
            
            actix::spawn(async move {
                match pool_clone.get().await {
                    Ok(client) => {
                        match db::get_rooms(&client).await {
                            Ok(rooms) => {
                                // Convert to RoomInfo objects
                                let room_infos: Vec<RoomInfo> = rooms.iter()
                                    .map(|room| {
                                        RoomInfo {
                                            id: room.id,
                                            name: room.name.clone(),
                                            room_type: format!("{:?}", room.room_type).to_lowercase(),
                                            member_count: 0, // We would need another query to get this
                                            is_protected: room.password_hash.is_some(),
                                        }
                                    })
                                    .collect();
                                
                                let room_list_message = ChatMessage {
                                    message_type: MessageType::RoomList,
                                    user: "system".to_string(),
                                    text: "Available rooms".to_string(),
                                    timestamp: Utc::now(),
                                    avatar: generate_avatar_url("system"),
                                    users: None,
                                    room_id: None,
                                    rooms: Some(room_infos),
                                    error: None,
                                };
                                
                                if let Ok(msg) = serde_json::to_string(&room_list_message) {
                                    addr.do_send(WsMessage(msg));
                                }
                            },
                            Err(e) => {
                                eprintln!("Error getting rooms: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error getting database client: {}", e);
                    }
                }
            });
        }

        // Send join notification to all users
        let join_message = ChatMessage {
            message_type: MessageType::Join,
            user: self.username.clone(),
            text: format!("{} joined the chat", self.username),
            timestamp: Utc::now(),
            avatar: generate_avatar_url(&self.username),
            users: None,
            room_id: None,
            rooms: None,
            error: None,
        };

        let msg = serde_json::to_string(&join_message).unwrap();
        for (_, addr) in connections.iter() {
            addr.do_send(WsMessage(msg.clone()));
        }
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // If user was in a room, remove them
        if let Some(room_id) = self.current_room_id {
            // Send room leave notification to other users in the room
            let leave_room_message = ChatMessage {
                message_type: MessageType::RoomLeft,
                user: self.username.clone(),
                text: format!("{} left the room", self.username),
                timestamp: Utc::now(),
                avatar: generate_avatar_url(&self.username),
                users: None,
                room_id: Some(room_id),
                rooms: None,
                error: None,
            };

            if let Ok(msg) = serde_json::to_string(&leave_room_message) {
                let connections = self.addr.lock().unwrap();
                for (username, addr) in connections.iter() {
                    if username != &self.username {
                        addr.do_send(WsMessage(msg.clone()));
                    }
                }
            }
        }

        // Remove user from global connections
        let mut connections = self.addr.lock().unwrap();
        connections.retain(|(_, addr)| addr != &ctx.address());

        // Send leave notification to all users
        let leave_message = ChatMessage {
            message_type: MessageType::Leave,
            user: self.username.clone(),
            text: format!("{} left the chat", self.username),
            timestamp: Utc::now(),
            avatar: generate_avatar_url(&self.username),
            users: None,
            room_id: None,
            rooms: None,
            error: None,
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
        // Special internal messages for user ID update
        if msg.0.contains("\"user_id_update\":") {
            // Parse the user ID from the message
            if let Some(id_str) = msg.0.split(':').nth(1) {
                if let Ok(user_id) = id_str.trim_end_matches('}').trim().parse::<i32>() {
                    self.user_id = Some(user_id);
                    println!("Updated user_id for {} to {}", self.username, user_id);
                }
            }
            return; // Don't send this message to the client
        }
        
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Special internal messages
                if text.contains("\"user_id_update\":") {
                    // Parse the user ID from the message
                    if let Some(id_str) = text.split(':').nth(1) {
                        if let Some(id_str) = id_str.trim_end_matches('}').trim().parse::<i32>().ok() {
                            self.user_id = Some(id_str);
                            println!("Updated user_id for {} to {}", self.username, id_str);
                        }
                    }
                    return; // Don't process this message further
                }
                
                println!("Received text message: {}", text);

                match serde_json::from_str::<ChatMessage>(&text) {
                    Ok(mut chat_message) => {
                        // Set common message properties
                        chat_message.user = self.username.clone();
                        chat_message.timestamp = Utc::now();
                        chat_message.avatar = generate_avatar_url(&self.username);

                        println!("Processing message type: {:?}", chat_message.message_type);

                        match chat_message.message_type {
                            MessageType::Chat => {
                                // Store message in database if user is in a room
                                if let Some(room_id) = self.current_room_id {
                                    chat_message.room_id = Some(room_id);
                                    
                                    // Save to database if connection exists
                                    if let Some(ref pool) = self.db_pool {
                                        if let Some(user_id) = self.user_id {
                                            let content = chat_message.text.clone();
                                            let pool_clone = pool.clone();
                                            let user_id_clone = user_id;
                                            let room_id_clone = room_id;
                                            
                                            actix::spawn(async move {
                                                if let Ok(client) = pool_clone.get().await {
                                                    if let Err(e) = db::save_message(&client, room_id_clone, user_id_clone, &content).await {
                                                        eprintln!("Error saving message: {}", e);
                                                    }
                                                }
                                            });
                                        }
                                    }
                                    
                                    // Broadcast to all users in the room
                                    if let Ok(msg) = serde_json::to_string(&chat_message) {
                                        let connections = self.addr.lock().unwrap();
                                        for (_, addr) in connections.iter() {
                                            // We'll send to all connections and let each client decide
                                            // whether to display based on their current room
                                            addr.do_send(WsMessage(msg.clone()));
                                        }
                                    }
                                } else {
                                    // Send to all users in global chat
                                    if let Ok(msg) = serde_json::to_string(&chat_message) {
                                        let connections = self.addr.lock().unwrap();
                                        for (_, addr) in connections.iter() {
                                            addr.do_send(WsMessage(msg.clone()));
                                        }
                                    }
                                }
                            },
                            MessageType::Typing | MessageType::StopTyping => {
                                // Send typing indicators to everyone except the sender
                                if let Ok(msg) = serde_json::to_string(&chat_message) {
                                    let connections = self.addr.lock().unwrap();
                                    
                                    for (username, addr) in connections.iter() {
                                        if username != &self.username {
                                            addr.do_send(WsMessage(msg.clone()));
                                        }
                                    }
                                }
                            },
                            MessageType::CreateRoom => {
                                // Parse the create room command from the text field
                                println!("Received CreateRoom message: {}", &chat_message.text);
                                if let Ok(command) = serde_json::from_str::<CreateRoomCommand>(&chat_message.text) {
                                    if let Some(ref pool) = self.db_pool {
                                        println!("User ID status: {:?}", self.user_id);
                                        
                                        // If user ID is not set, try to get it from the database
                                        if self.user_id.is_none() {
                                            let pool_clone = pool.clone();
                                            let username_clone = self.username.clone();
                                            let addr = ctx.address();
                                            let command_clone = command.clone();
                                            
                                            actix::spawn(async move {
                                                if let Ok(client) = pool_clone.get().await {
                                                    match db::get_user_by_username(&client, &username_clone).await {
                                                        Ok(Some((user_id, _))) => {
                                                            println!("Found user {} with ID {}", username_clone, user_id);
                                                            
                                                            // Now create the room
                                                            handle_create_room(
                                                                &client,
                                                                &command_clone,
                                                                user_id,
                                                                username_clone,
                                                                addr
                                                            ).await;
                                                        },
                                                        _ => {
                                                            // Create a new user
                                                            let random_password = format!("pass_{}", rand::random::<u32>());
                                                            match db::create_user(&client, &username_clone, &random_password).await {
                                                                Ok(user_id) => {
                                                                    println!("Created new user {} with ID {}", username_clone, user_id);
                                                                    
                                                                    // Now create the room
                                                                    handle_create_room(
                                                                        &client,
                                                                        &command_clone,
                                                                        user_id,
                                                                        username_clone, 
                                                                        addr
                                                                    ).await;
                                                                },
                                                                Err(e) => {
                                                                    eprintln!("Error creating user {}: {}", username_clone, e);
                                                                    
                                                                    let error_message = ChatMessage {
                                                                        message_type: MessageType::Error,
                                                                        user: "system".to_string(),
                                                                        text: "Failed to create user for room creation".to_string(),
                                                                        timestamp: Utc::now(),
                                                                        avatar: generate_avatar_url("system"),
                                                                        users: None,
                                                                        room_id: None,
                                                                        rooms: None,
                                                                        error: Some(format!("Database error: {}", e)),
                                                                    };
                                                                    
                                                                    if let Ok(msg) = serde_json::to_string(&error_message) {
                                                                        addr.do_send(WsMessage(msg));
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                        } else {
                                            let user_id = self.user_id.unwrap();
                                            println!("Creating room '{}' as user_id: {}", command.name, user_id);
                                            let pool_clone = pool.clone();
                                            let username_clone = self.username.clone();
                                            let user_id_clone = user_id;
                                            let command_clone = command.clone();
                                            let addr = ctx.address();
                                            
                                            actix::spawn(async move {
                                                if let Ok(client) = pool_clone.get().await {
                                                    handle_create_room(
                                                        &client,
                                                        &command_clone,
                                                        user_id_clone,
                                                        username_clone,
                                                        addr
                                                    ).await;
                                                }
                                            });
                                        }
                                    }
                                } else {
                                    println!("Failed to parse CreateRoomCommand: {}", &chat_message.text);
                                }
                            },
                            MessageType::JoinRoom => {
                                // Parse the join room command from the text field
                                if let Ok(command) = serde_json::from_str::<JoinRoomCommand>(&chat_message.text) {
                                    if let Some(ref pool) = self.db_pool {
                                        if let Some(user_id) = self.user_id {
                                            let pool_clone = pool.clone();
                                            let user_id_clone = user_id;
                                            let room_id = command.room_id;
                                            let username_clone = self.username.clone();
                                            let addr = ctx.address();
                                            
                                            // Set the current room ID
                                            self.current_room_id = Some(room_id);
                                            
                                            actix::spawn(async move {
                                                if let Ok(client) = pool_clone.get().await {
                                                    match db::join_room(&client, room_id, user_id_clone).await {
                                                        Ok(_) => {
                                                            println!("User {} joined room {}", username_clone, room_id);
                                                            
                                                            // Get room history
                                                            match db::get_room_messages(&client, room_id, 50).await {
                                                                Ok(messages) => {
                                                                    for msg in messages {
                                                                        // We need to get the username for the sender_id
                                                                        let sender_username = if msg.sender_id == user_id_clone {
                                                                            username_clone.clone()
                                                                        } else {
                                                                            // Ideally, we'd query the database here to get the username
                                                                            format!("User_{}", msg.sender_id)
                                                                        };
                                                                        
                                                                        let avatar = generate_avatar_url(&sender_username);
                                                                        
                                                                        let chat_message = ChatMessage {
                                                                            message_type: MessageType::Chat,
                                                                            user: sender_username,
                                                                            text: msg.content,
                                                                            timestamp: msg.created_at,
                                                                            avatar,
                                                                            users: None,
                                                                            room_id: Some(room_id),
                                                                            rooms: None,
                                                                            error: None,
                                                                        };
                                                                        
                                                                        if let Ok(msg_str) = serde_json::to_string(&chat_message) {
                                                                            addr.do_send(WsMessage(msg_str));
                                                                        }
                                                                    }
                                                                },
                                                                Err(e) => {
                                                                    eprintln!("Error getting room messages: {}", e);
                                                                }
                                                            }
                                                            
                                                            // Notify all users in the room
                                                            let join_room_message = ChatMessage {
                                                                message_type: MessageType::RoomJoined,
                                                                user: username_clone.clone(),
                                                                text: format!("{} joined the room", username_clone),
                                                                timestamp: Utc::now(),
                                                                avatar: generate_avatar_url(&username_clone),
                                                                users: None,
                                                                room_id: Some(room_id),
                                                                rooms: None,
                                                                error: None,
                                                            };
                                                            
                                                            if let Ok(msg) = serde_json::to_string(&join_room_message) {
                                                                addr.do_send(WsMessage(msg));
                                                            }
                                                        },
                                                        Err(e) => {
                                                            eprintln!("Error joining room: {}", e);
                                                            
                                                            let error_message = ChatMessage {
                                                                message_type: MessageType::Error,
                                                                user: "system".to_string(),
                                                                text: "Failed to join room".to_string(),
                                                                timestamp: Utc::now(),
                                                                avatar: generate_avatar_url("system"),
                                                                users: None,
                                                                room_id: None,
                                                                rooms: None,
                                                                error: Some(format!("Database error: {}", e)),
                                                            };
                                                            
                                                            if let Ok(msg) = serde_json::to_string(&error_message) {
                                                                addr.do_send(WsMessage(msg));
                                                            }
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }
                                }
                            },
                            MessageType::LeaveRoom => {
                                if let Some(room_id) = self.current_room_id {
                                    // Send leave notification to users in the room
                                    let leave_room_message = ChatMessage {
                                        message_type: MessageType::RoomLeft,
                                        user: self.username.clone(),
                                        text: format!("{} left the room", self.username),
                                        timestamp: Utc::now(),
                                        avatar: generate_avatar_url(&self.username),
                                        users: None,
                                        room_id: Some(room_id),
                                        rooms: None,
                                        error: None,
                                    };
                                    
                                    if let Ok(msg) = serde_json::to_string(&leave_room_message) {
                                        let connections = self.addr.lock().unwrap();
                                        for (username, addr) in connections.iter() {
                                            if username != &self.username {
                                                addr.do_send(WsMessage(msg.clone()));
                                            }
                                        }
                                    }
                                    
                                    // Clear the current room
                                    self.current_room_id = None;
                                }
                            },
                            _ => {
                                // Forward the message to all users
                                if let Ok(msg) = serde_json::to_string(&chat_message) {
                                    let connections = self.addr.lock().unwrap();
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

// Helper function to create a room
async fn handle_create_room(
    client: &deadpool_postgres::Client,
    command: &CreateRoomCommand,
    user_id: i32,
    username: String,
    addr: actix::Addr<ChatSession>
) {
    match db::create_room(
        client, 
        &command.name, 
        &command.room_type, 
        command.password.as_deref(),
        user_id
    ).await {
        Ok(room_id) => {
            println!("Room created with ID: {}", room_id);
            
            // Send room list update to all users
            match db::get_rooms(client).await {
                Ok(rooms) => {
                    let room_infos: Vec<RoomInfo> = rooms.iter()
                        .map(|room| {
                            RoomInfo {
                                id: room.id,
                                name: room.name.clone(),
                                room_type: format!("{:?}", room.room_type).to_lowercase(),
                                member_count: 0,
                                is_protected: room.password_hash.is_some(),
                            }
                        })
                        .collect();
                    
                    let room_list_message = ChatMessage {
                        message_type: MessageType::RoomList,
                        user: "system".to_string(),
                        text: format!("{} created a new room: {}", username, command.name),
                        timestamp: Utc::now(),
                        avatar: generate_avatar_url("system"),
                        users: None,
                        room_id: None,
                        rooms: Some(room_infos),
                        error: None,
                    };
                    
                    if let Ok(msg) = serde_json::to_string(&room_list_message) {
                        addr.do_send(WsMessage(msg));
                    }
                },
                Err(e) => {
                    eprintln!("Error getting rooms: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Error creating room: {}", e);
            
            let error_message = ChatMessage {
                message_type: MessageType::Error,
                user: "system".to_string(),
                text: "Failed to create room".to_string(),
                timestamp: Utc::now(),
                avatar: generate_avatar_url("system"),
                users: None,
                room_id: None,
                rooms: None,
                error: Some(format!("Database error: {}", e)),
            };
            
            if let Ok(msg) = serde_json::to_string(&error_message) {
                addr.do_send(WsMessage(msg));
            }
        }
    }
}