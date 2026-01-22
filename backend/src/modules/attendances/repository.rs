// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::attendances::{self, Entity as Attendance};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

#[derive(Clone)]
pub struct AttendanceRepository {
    db: Database,
}

impl AttendanceRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new attendance
    pub async fn create(
        &self,
        active_model: attendances::ActiveModel,
    ) -> Result<attendances::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<attendances::Model>, AppError> {
        Attendance::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: i64,
    ) -> Result<(Vec<attendances::Model>, u64), AppError> {
        let mut query =
            Attendance::find().filter(attendances::Column::FoundationId.eq(foundation_id));
        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "date" => {
                    if is_desc {
                        query.order_by_desc(attendances::Column::Date)
                    } else {
                        query.order_by_asc(attendances::Column::Date)
                    }
                }
                _ => query.order_by_desc(attendances::Column::Date),
            };
        } else {
            query = query.order_by_desc(attendances::Column::Date);
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
    pub async fn find_by_date(
        &self,
        date: &str,
        foundation_id: i64,
    ) -> Result<Option<attendances::Model>, AppError> {
        Attendance::find()
            .filter(attendances::Column::FoundationId.eq(foundation_id))
            .filter(attendances::Column::Date.eq(date))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update attendance
    pub async fn update(
        &self,
        id: i64,
        active_model: attendances::ActiveModel,
    ) -> Result<attendances::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete attendance
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Attendance::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
