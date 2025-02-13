    use tokio_postgres::Client;
    

    pub async fn create_tables(client: &Client) -> Result<(), tokio_postgres::Error> {
        client.batch_execute("
            DROP TABLE IF EXISTS messages CASCADE;
            DROP TABLE IF EXISTS room_members CASCADE;
            DROP TABLE IF EXISTS rooms CASCADE;

            CREATE TABLE rooms (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                room_type VARCHAR(50) NOT NULL,
                password_hash VARCHAR(255),
                created_by VARCHAR(255) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE room_members (
                room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE,
                user_id VARCHAR(255) NOT NULL,
                role VARCHAR(50) DEFAULT 'member',
                joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (room_id, user_id)
            );

            CREATE TABLE messages (
                id SERIAL PRIMARY KEY,
                    room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE,
                    sender_id VARCHAR(255) NOT NULL,
                    content TEXT NOT NULL,
                    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
                );  
                CREATE INDEX messages_room_id_created_at_idx ON messages(room_id, created_at);
        ").await
    }