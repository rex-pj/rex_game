use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct SeaOrmConnection {
    pub pool: Arc<DatabaseConnection>,
}

impl SeaOrmConnection {
    // Create a connection pool
    pub async fn new(database_url: &str) -> Result<SeaOrmConnection, DbErr> {
        let pool = Database::connect(database_url).await;
        match pool {
            Ok(_) => Ok(SeaOrmConnection {
                pool: Arc::new(pool.unwrap()),
            }),
            Err(err) => {
                return Err(err);
            }
        }
    }
}
