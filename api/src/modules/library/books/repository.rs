// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::filters::global_search::GlobalSearch;
use crate::utils::pagination::PaginationParams;
use entity::books::{self, Entity as Book};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QueryTrait, Set,
};

#[derive(Clone)]
pub struct BookRepository {
    db: Database,
}

impl BookRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    pub async fn create(
        &self,
        active_model: books::ActiveModel,
    ) -> Result<books::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<books::Model>, AppError> {
        Book::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_all_old(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<books::Model>, u64), AppError> {
        let mut query = Book::find();

        if let Some(fid) = foundation_id {
            query = query.filter(books::Column::FoundationId.eq(fid));
        }

        if let Some(ref search) = params.search {
            query = query.filter(
                Condition::any()
                    .add(books::Column::Title.contains(search))
                    .add(books::Column::Isbn.contains(search)),
            );
        }

        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "title" | "name" => {
                    if is_desc {
                        query.order_by_desc(books::Column::Title)
                    } else {
                        query.order_by_asc(books::Column::Title)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(books::Column::CreatedAt)
                    } else {
                        query.order_by_asc(books::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(books::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(books::Column::CreatedAt);
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
        foundation_id: Option<i64>,
    ) -> Result<(Vec<books::Model>, u64), AppError> {
        let mut query = Book::find();

        if let Some(fid) = foundation_id {
            query = query.filter(books::Column::FoundationId.eq(fid));
        }

        let db_name = self.db.get_db_name()?;

        query = GlobalSearch::apply_from_entity::<Book>(
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
                "title" | "name" => {
                    if is_desc {
                        query.order_by_desc(books::Column::Title)
                    } else {
                        query.order_by_asc(books::Column::Title)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(books::Column::CreatedAt)
                    } else {
                        query.order_by_asc(books::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(books::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(books::Column::CreatedAt);
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

    pub async fn find_by_title(
        &self,
        title: &str,
        foundation_id: i64,
    ) -> Result<Option<books::Model>, AppError> {
        Book::find()
            .filter(books::Column::FoundationId.eq(foundation_id))
            .filter(books::Column::Title.eq(title))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn update(
        &self,
        id: i64,
        active_model: books::ActiveModel,
    ) -> Result<books::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Book::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
