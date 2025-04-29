use tokio_postgres::Client;

pub async fn create_tables(client: &Client) -> Result<(), tokio_postgres::Error> {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255),
            avatar_url VARCHAR(255),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS rooms (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            \"type\" VARCHAR(50) NOT NULL CHECK (\"type\" IN ('public', 'private', 'protected')),
            password_hash VARCHAR(255),
            created_by INTEGER REFERENCES users(id),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS room_members (
            room_id INTEGER REFERENCES rooms(id),
            user_id INTEGER REFERENCES users(id),
            role VARCHAR(50) DEFAULT 'member' CHECK (role IN ('admin', 'member')),
            joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (room_id, user_id)
        );

        CREATE TABLE IF NOT EXISTS messages (
            id SERIAL PRIMARY KEY,
            room_id INTEGER REFERENCES rooms(id),
            sender_id INTEGER REFERENCES users(id),
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create indexes for better performance
        CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
        CREATE INDEX IF NOT EXISTS idx_rooms_name ON rooms(name);
        CREATE INDEX IF NOT EXISTS idx_rooms_type ON rooms(\"type\");
        CREATE INDEX IF NOT EXISTS idx_room_members_room_id ON room_members(room_id);
        CREATE INDEX IF NOT EXISTS idx_room_members_user_id ON room_members(user_id);
        CREATE INDEX IF NOT EXISTS idx_messages_room_id ON messages(room_id);
        CREATE INDEX IF NOT EXISTS idx_messages_sender_id ON messages(sender_id);
        CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);
    ").await
}