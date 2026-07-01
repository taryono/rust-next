// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::settings::{self, Entity as Setting};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct SettingRepository {
    db: Database,
}

impl SettingRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new setting
    pub async fn create(
        &self,
        active_model: settings::ActiveModel,
    ) -> Result<settings::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<settings::Model>, AppError> {
        Setting::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<settings::Model>, u64), AppError> {
        let mut query = Setting::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(settings::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(settings::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::Name)
                    } else {
                        query.order_by_asc(settings::Column::Name)
                    }
                }
                "active" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::IsActive)
                    } else {
                        query.order_by_asc(settings::Column::IsActive)
                    }
                }
                "start_date" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::StartDate)
                    } else {
                        query.order_by_asc(settings::Column::StartDate)
                    }
                }
                "end_date" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::EndDate)
                    } else {
                        query.order_by_asc(settings::Column::EndDate)
                    }
                }
                "academic_calendar_id" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::AcademicYearId)
                    } else {
                        query.order_by_asc(settings::Column::AcademicYearId)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(settings::Column::CreatedAt)
                    } else {
                        query.order_by_asc(settings::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(settings::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(settings::Column::CreatedAt);
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
    ) -> Result<Option<settings::Model>, AppError> {
        Setting::find()
            .filter(settings::Column::FoundationId.eq(foundation_id))
            .filter(settings::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update setting
    pub async fn update(
        &self,
        id: i64,
        active_model: settings::ActiveModel,
    ) -> Result<settings::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete setting
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Setting::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
