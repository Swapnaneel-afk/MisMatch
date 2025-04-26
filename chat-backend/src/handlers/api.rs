use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use deadpool_postgres::Pool;
use crate::db::models::{User, Room, Message};
use crate::utils::password::hash_password;

// Request/Response Structs
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub room_type: String, // "public", "private", "protected"
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct JoinRoomRequest {
    pub user_id: i32,
    pub room_id: i32,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

// User API Handlers
pub async fn create_user(
    pool: web::Data<Pool>,
    user_data: web::Json<CreateUserRequest>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    };

    // Check if username already exists
    match User::find_by_username(&client, &user_data.username).await {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                success: false,
                message: Some("Username already exists".to_string()),
                data: None,
            });
        }
        Ok(None) => {
            // Username available, continue
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    }

    // Hash the password if provided
    let password_hash = user_data.password.as_ref().map(|password| hash_password(password));

    // Create the user
    let new_user = User {
        id: None,
        username: user_data.username.clone(),
        password_hash,
        avatar_url: user_data.avatar_url.clone(),
        created_at: None,
    };

    match User::create(&client, &new_user).await {
        Ok(created_user) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: Some("User created successfully".to_string()),
            data: Some(created_user),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: Some(format!("Failed to create user: {}", e)),
            data: None,
        }),
    }
}

// Room API Handlers
pub async fn create_room(
    pool: web::Data<Pool>,
    user_id: web::Path<i32>,
    room_data: web::Json<CreateRoomRequest>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    };

    // Hash the room password if it's a protected room
    let password_hash = if room_data.room_type == "protected" {
        room_data.password.as_ref().map(|password| hash_password(password))
    } else {
        None
    };

    // Create room
    let new_room = Room {
        id: None,
        name: room_data.name.clone(),
        room_type: room_data.room_type.clone(),
        password_hash,
        created_by: Some(*user_id),
        created_at: None,
    };

    match Room::create(&client, &new_room).await {
        Ok(created_room) => {
            // Add the creator as an admin member of the room
            if let Some(room_id) = created_room.id {
                if let Err(e) = Room::join_room(&client, *user_id, room_id, "admin").await {
                    // Log error but don't fail the request
                    eprintln!("Failed to add user as admin to room: {}", e);
                }
            }

            HttpResponse::Ok().json(ApiResponse {
                success: true,
                message: Some("Room created successfully".to_string()),
                data: Some(created_room),
            })
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: Some(format!("Failed to create room: {}", e)),
            data: None,
        }),
    }
}

pub async fn get_rooms(
    pool: web::Data<Pool>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    };

    match Room::find_all(&client).await {
        Ok(rooms) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: None,
            data: Some(rooms),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: Some(format!("Failed to fetch rooms: {}", e)),
            data: None,
        }),
    }
}

pub async fn join_room(
    pool: web::Data<Pool>,
    join_data: web::Json<JoinRoomRequest>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    };

    // In a real app, you'd check the room password if it's protected

    match Room::join_room(&client, join_data.user_id, join_data.room_id, "member").await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            success: true,
            message: Some("Joined room successfully".to_string()),
            data: None,
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: Some(format!("Failed to join room: {}", e)),
            data: None,
        }),
    }
}

// Message API handlers
pub async fn get_room_messages(
    pool: web::Data<Pool>,
    room_id: web::Path<i32>,
) -> impl Responder {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                success: false,
                message: Some(format!("Database error: {}", e)),
                data: None,
            });
        }
    };

    // Fetch last 50 messages
    match Message::find_by_room(&client, *room_id, 50).await {
        Ok(messages) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            message: None,
            data: Some(messages),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            success: false,
            message: Some(format!("Failed to fetch messages: {}", e)),
            data: None,
        }),
    }
} 