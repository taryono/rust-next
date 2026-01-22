// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::teachers::{self, Entity as Teacher};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct TeacherRepository {
    db: Database,
}

impl TeacherRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new teacher
    pub async fn create(
        &self,
        active_model: teachers::ActiveModel,
    ) -> Result<teachers::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<teachers::Model>, AppError> {
        Teacher::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<teachers::Model>, u64), AppError> {
        let mut query = Teacher::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(teachers::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(teachers::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(teachers::Column::Name)
                    } else {
                        query.order_by_asc(teachers::Column::Name)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(teachers::Column::CreatedAt)
                    } else {
                        query.order_by_asc(teachers::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(teachers::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(teachers::Column::CreatedAt);
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
    pub async fn find_by_name(
        &self,
        name: &str,
        foundation_id: i64,
    ) -> Result<Option<teachers::Model>, AppError> {
        Teacher::find()
            .filter(teachers::Column::FoundationId.eq(foundation_id))
            .filter(teachers::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update teacher
    pub async fn update(
        &self,
        id: i64,
        active_model: teachers::ActiveModel,
    ) -> Result<teachers::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete teacher
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Teacher::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
