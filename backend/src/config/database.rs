// backend/src/config/database.rs
use crate::errors::AppError;
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
            connection: Arc::new(connection),
        })
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub fn get_db_name(&self) -> Result<String, AppError> {
        std::env::var("DATABASE_URL")
            .ok()
            .and_then(|url| url.rsplit('/').next().map(String::from))
            .ok_or_else(|| AppError::InternalServerError("Cannot parse DB name".into()))
    }
}
