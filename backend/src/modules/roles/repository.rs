// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::roles::{self, Entity as Role};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct RoleRepository {
    db: Database,
}

impl RoleRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new role
    pub async fn create(&self, active_model: roles::ActiveModel) -> Result<roles::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<roles::Model>, AppError> {
        Role::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<roles::Model>, u64), AppError> {
        let mut query = Role::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(roles::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(roles::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(roles::Column::Name)
                    } else {
                        query.order_by_asc(roles::Column::Name)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(roles::Column::CreatedAt)
                    } else {
                        query.order_by_asc(roles::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(roles::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(roles::Column::CreatedAt);
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
    ) -> Result<Option<roles::Model>, AppError> {
        Role::find()
            .filter(roles::Column::FoundationId.eq(foundation_id))
            .filter(roles::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }
    /// Update role
    pub async fn update(
        &self,
        id: i64,
        active_model: roles::ActiveModel,
    ) -> Result<roles::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete role
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Role::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
