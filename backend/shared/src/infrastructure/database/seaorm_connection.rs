use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct SeaOrmConnection {
    pub pool: Arc<DatabaseConnection>,
}

impl SeaOrmConnection {
    // Create a connection pool
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let db_pool = Database::connect(database_url).await;
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
