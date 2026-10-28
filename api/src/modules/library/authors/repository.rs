// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::filters::global_search::GlobalSearch;
use crate::utils::pagination::PaginationParams;
use entity::authors::{self, Entity as Author};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QueryTrait, Set,
};

#[derive(Clone)]
pub struct AuthorRepository {
    db: Database,
}

pub type BookRepository = AuthorRepository;

impl AuthorRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    pub async fn create(
        &self,
        active_model: authors::ActiveModel,
    ) -> Result<authors::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<authors::Model>, AppError> {
        Author::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_all_old(
        &self,
        params: &PaginationParams,
        _foundation_id: Option<i64>,
    ) -> Result<(Vec<authors::Model>, u64), AppError> {
        let mut query = Author::find();

        if let Some(ref search) = params.search {
            query = query.filter(Condition::any().add(authors::Column::FullName.contains(search)));
        }

        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "full_name" | "name" => {
                    if is_desc {
                        query.order_by_desc(authors::Column::FullName)
                    } else {
                        query.order_by_asc(authors::Column::FullName)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(authors::Column::CreatedAt)
                    } else {
                        query.order_by_asc(authors::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(authors::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(authors::Column::CreatedAt);
        }

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

    pub async fn find_all(
        &self,
        params: &PaginationParams,
        _foundation_id: Option<i64>,
    ) -> Result<(Vec<authors::Model>, u64), AppError> {
        let mut query = Author::find();

        let db_name = self.db.get_db_name()?;

        query = GlobalSearch::apply_from_entity::<Author>(
            self.conn(),
            query,
            &db_name,
            params.search.as_deref(),
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "full_name" | "name" => {
                    if is_desc {
                        query.order_by_desc(authors::Column::FullName)
                    } else {
                        query.order_by_asc(authors::Column::FullName)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(authors::Column::CreatedAt)
                    } else {
                        query.order_by_asc(authors::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(authors::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(authors::Column::CreatedAt);
        }

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

    pub async fn find_by_full_name(
        &self,
        full_name: &str,
    ) -> Result<Option<authors::Model>, AppError> {
        Author::find()
            .filter(authors::Column::FullName.eq(full_name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn update(
        &self,
        id: i64,
        active_model: authors::ActiveModel,
    ) -> Result<authors::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Author::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
