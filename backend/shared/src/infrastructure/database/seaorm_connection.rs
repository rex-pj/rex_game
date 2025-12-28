use std::sync::Arc;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub struct SeaOrmConnection {
    pub pool: Arc<DatabaseConnection>,
}

impl SeaOrmConnection {
    // Create a connection pool with optimized settings
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let mut opt = ConnectOptions::new(database_url.to_string());

        // Configure connection pool parameters
        opt.max_connections(100) // Maximum number of connections in the pool
            .min_connections(5) // Minimum number of connections to maintain
            .connect_timeout(Duration::from_secs(8)) // Timeout for establishing a new connection
            .acquire_timeout(Duration::from_secs(8)) // Timeout for acquiring a connection from the pool
            .idle_timeout(Duration::from_secs(300)) // Close connections idle for 5 minutes
            .max_lifetime(Duration::from_secs(1800)) // Close connections after 30 minutes
            .sqlx_logging(true) // Enable SQL query logging for debugging
            .sqlx_logging_level(log::LevelFilter::Debug); // Set logging level

        let db_pool = Database::connect(opt).await;
        match db_pool {
            Ok(pool) => Ok(Self {
                pool: Arc::new(pool),
            }),
            Err(err) => {
                return Err(err);
            }
        }
    }
}
