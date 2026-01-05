use mongodb::{Client, Database};
use std::env;

pub struct DatabaseConnection {
    pub client: Client,
    pub database: Database,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load environment variables
        dotenv::dotenv().ok();

        // 3. Sumber Alamat (env::var("MONGODB_URI"))
        let mongodb_uri =
            env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());

        // 4. Nama Database (env::var("DATABASE_NAME"))
        let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "todo_db".to_string());

        // Create MongoDB client
        //1. Koneksi Klien MongoDB (Client::with_uri_str)
        let client = Client::with_uri_str(&mongodb_uri).await?;

        //2. Seleksi Database Spesifik (client.database)
        let database = client.database(&database_name);

        Ok(DatabaseConnection { client, database })
    }
}
