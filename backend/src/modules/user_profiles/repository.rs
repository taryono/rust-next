// backend/src/modules/user_profiles/repository.rs

// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::user_profiles::{self, Entity as UserProfile};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Clone)]
pub struct UserProfileRepository {
    db: Database,
}

impl UserProfileRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new user_profile
    pub async fn create(
        &self,
        active_model: user_profiles::ActiveModel,
    ) -> Result<user_profiles::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<user_profiles::Model>, AppError> {
        UserProfile::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<user_profiles::Model>, u64), AppError> {
        let mut query = UserProfile::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(user_profiles::Column::FoundationId.eq(fid));
        }
        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(user_profiles::Column::CreatedAt)
                    } else {
                        query.order_by_asc(user_profiles::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(user_profiles::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(user_profiles::Column::CreatedAt);
        }

        // Paginate dengan validasi
        let per_page = params.per_page();
        let paginator = query.paginate(self.conn(), per_page as u64);

        let total = paginator
            .num_items()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let items = paginator
            .fetch_page(params.page() - 1)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok((items, total.try_into().unwrap_or(0)))
    }
    /// Find by name within a foundation
    pub async fn find_by_user(
        &self,
        user_id: &str,
        foundation_id: i64,
    ) -> Result<Option<user_profiles::Model>, AppError> {
        UserProfile::find()
            .filter(user_profiles::Column::FoundationId.eq(foundation_id))
            .filter(user_profiles::Column::UserId.eq(user_id))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Update user_profile
    pub async fn update(
        &self,
        id: i64,
        active_model: user_profiles::ActiveModel,
    ) -> Result<user_profiles::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete user_profile
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        UserProfile::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
