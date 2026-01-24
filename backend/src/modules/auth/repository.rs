// src/modules/auth/repository.rs
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::password;
use entity::roles::{self as roles, Entity as Roles};
use entity::users::{self as users, Entity as User};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set};
#[derive(Clone)]
pub struct AuthRepository {
    db: Database,
}

impl AuthRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, AppError> {
        User::find()
            .filter(users::Column::Email.eq(email))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<users::Model>, AppError> {
        User::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find user by ID with roles
    pub async fn find_by_id_with_roles(
        &self,
        id: i64,
    ) -> Result<Option<(users::Model, Vec<roles::Model>)>, AppError> {
        User::find_by_id(id)
            .find_with_related(Roles)
            .all(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
            .map(|mut res| res.pop())
    }

    /// Find user by email with roles
    pub async fn find_by_email_with_roles(
        &self,
        email: &str,
    ) -> Result<Option<(users::Model, Vec<roles::Model>)>, AppError> {
        User::find()
            .filter(users::Column::Email.eq(email))
            .find_with_related(Roles)
            .all(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
            .map(|mut res| res.pop())
    }

    /// Create new user
    pub async fn create_user(
        &self,
        name: String,
        email: String,
        password: String,
    ) -> Result<users::Model, AppError> {
        // Hash password
        let hashed_password = password::hash(&password)?;

        let new_user = users::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(hashed_password),
            ..Default::default()
        };

        new_user
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        User::find()
            .filter(users::Column::Email.eq(email))
            .count(self.conn())
            .await
            .map(|count| count > 0)
            .map_err(AppError::from)
    }
}
