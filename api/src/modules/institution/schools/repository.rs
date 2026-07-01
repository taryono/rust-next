// api/src/modules/institution/school/repository.rs
// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::schools::{self, Entity as School};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct SchoolRepository {
    db: Database,
}

impl SchoolRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new class
    pub async fn create(
        &self,
        active_model: schools::ActiveModel,
    ) -> Result<schools::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<schools::Model>, AppError> {
        School::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<schools::Model>, u64), AppError> {
        let mut query = School::find();
        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(schools::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(schools::Column::Name)
                    } else {
                        query.order_by_asc(schools::Column::Name)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(schools::Column::CreatedAt)
                    } else {
                        query.order_by_asc(schools::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(schools::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(schools::Column::CreatedAt);
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
    pub async fn find_by_name(&self, name: &str) -> Result<Option<schools::Model>, AppError> {
        School::find()
            .filter(schools::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update class
    pub async fn update(
        &self,
        id: i64,
        active_model: schools::ActiveModel,
    ) -> Result<schools::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete class
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        School::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
