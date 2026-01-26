// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::modules::users::dto::UserResponse;
use crate::utils::pagination::PaginationParams;
use entity::{
    role_users::{self, Entity as RoleUser},
    roles::{self, Entity as Role},
    users::{self, Entity as User},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Clone)]
pub struct UserRepository {
    db: Database,
}

impl UserRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new user
    pub async fn create(&self, active_model: users::ActiveModel) -> Result<users::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn update(
        &self,
        id: i64,
        active_model: users::ActiveModel,
    ) -> Result<users::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);

        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<users::Model>, AppError> {
        User::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<users::Model>, u64), AppError> {
        let mut query = User::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(users::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(users::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(users::Column::Name)
                    } else {
                        query.order_by_asc(users::Column::Name)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(users::Column::CreatedAt)
                    } else {
                        query.order_by_asc(users::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(users::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(users::Column::CreatedAt);
        }

        // Paginate
        let paginator = query.paginate(self.conn(), params.per_page() as u64);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let items = paginator
            .fetch_page((params.page() - 1) as u64)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok((items, total.try_into().unwrap_or(0)))
    }

    /// Find active user for a foundation
    pub async fn find_active(&self, foundation_id: i64) -> Result<Option<users::Model>, AppError> {
        User::find()
            .filter(users::Column::FoundationId.eq(foundation_id))
            .filter(users::Column::IsActive.eq(1))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by name within a foundation
    pub async fn find_by_name(
        &self,
        name: &str,
        foundation_id: i64,
    ) -> Result<Option<users::Model>, AppError> {
        User::find()
            .filter(users::Column::FoundationId.eq(foundation_id))
            .filter(users::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by name within a foundation
    pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, AppError> {
        User::find()
            .filter(users::Column::Email.eq(email))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete user
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        User::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Delete user
    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        let user = User::find_by_id(id)
            .one(self.conn())
            .await?
            // .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        let mut model = user.into_active_model();
        model.deleted_at = Set(Some(chrono::Utc::now()));
        model.update(self.conn()).await?;
        Ok(())
    }

    pub async fn find_role_not_deleted(
        &self,
        role_id: i64,
    ) -> Result<Option<roles::Model>, AppError> {
        Role::find()
            .filter(roles::Column::Id.eq(role_id))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_not_deleted(&self, user_id: i64) -> Result<Option<users::Model>, AppError> {
        User::find()
            .filter(users::Column::Id.eq(user_id))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Restore user
    pub async fn restore(&self, id: i64) -> Result<UserResponse, AppError> {
        let user = User::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::not_found("User not found".to_string()))?;

        // ✅ Validasi: cek apakah memang sudah di-delete
        if user.deleted_at.is_none() {
            return Err(AppError::bad_request("User is not deleted".to_string()));
        }

        let mut model = user.into_active_model();
        model.deleted_at = Set(None); // ✅ Set ke None untuk restore
        model.updated_at = Set(chrono::Utc::now()); // ✅ Update timestamp

        let restored_user = model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(UserResponse::from(restored_user))
    }

    // Di users/repository.rs
    pub async fn remove_role_from_user(
        &self,
        db: &DatabaseConnection,
        user_id: i64,
        role_id: i64,
    ) -> Result<(), AppError> {
        let result = RoleUser::delete_many()
            .filter(role_users::Column::UserId.eq(user_id))
            .filter(role_users::Column::RoleId.eq(role_id))
            .exec(db)
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFoundError(
                "Role assignment not found".to_string(),
            ));
        }

        Ok(())
    }
}
