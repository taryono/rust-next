// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::rooms::{self, Entity as Room};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct RoomRepository {
    db: Database,
}

impl RoomRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new room
    pub async fn create(&self, active_model: rooms::ActiveModel) -> Result<rooms::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<rooms::Model>, AppError> {
        Room::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<rooms::Model>, u64), AppError> {
        let mut query = Room::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(rooms::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(rooms::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(rooms::Column::Name)
                    } else {
                        query.order_by_asc(rooms::Column::Name)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(rooms::Column::CreatedAt)
                    } else {
                        query.order_by_asc(rooms::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(rooms::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(rooms::Column::CreatedAt);
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
    ) -> Result<Option<rooms::Model>, AppError> {
        Room::find()
            .filter(rooms::Column::FoundationId.eq(foundation_id))
            .filter(rooms::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update room
    pub async fn update(
        &self,
        id: i64,
        active_model: rooms::ActiveModel,
    ) -> Result<rooms::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete room
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Room::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
