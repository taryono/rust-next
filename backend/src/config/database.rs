use sea_orm::{Database as SeaDatabase, DatabaseConnection, DbErr};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    pub connection: Arc<DatabaseConnection>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, DbErr> {
        let connection = SeaDatabase::connect(database_url).await?;
        
        log::info!("Database connection established");
        
        Ok(Self { 
            connection: Arc::new(connection) 
        })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}