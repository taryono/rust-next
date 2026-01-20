// ============================================================================
// repository.rs - Database Operations Only
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::academic_years::{self, Entity as AcademicYear};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

#[derive(Clone)]
pub struct AcademicYearRepository {
    db: Database,
}

impl AcademicYearRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Helper method untuk akses connection
    fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    /// Create new academic year
    pub async fn create(
        &self,
        active_model: academic_years::ActiveModel,
    ) -> Result<academic_years::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by ID
    pub async fn find_by_id(&self, id: u64) -> Result<Option<academic_years::Model>, AppError> {
        AcademicYear::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find all with pagination and filters
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<u64>,
    ) -> Result<(Vec<academic_years::Model>, u64), AppError> {
        let mut query = AcademicYear::find();

        // Filter by foundation_id if provided
        if let Some(fid) = foundation_id {
            query = query.filter(academic_years::Column::FoundationId.eq(fid));
        }

        // Apply search filter if provided
        if let Some(ref search) = params.search {
            query =
                query.filter(Condition::any().add(academic_years::Column::Name.contains(search)));
        }

        // Apply sorting
        if let Some(ref sort_by) = params.sort_by {
            let is_desc = params.sort_order.as_deref() == Some("desc");

            query = match sort_by.as_str() {
                "name" => {
                    if is_desc {
                        query.order_by_desc(academic_years::Column::Name)
                    } else {
                        query.order_by_asc(academic_years::Column::Name)
                    }
                }
                "start_date" => {
                    if is_desc {
                        query.order_by_desc(academic_years::Column::StartDate)
                    } else {
                        query.order_by_asc(academic_years::Column::StartDate)
                    }
                }
                "created_at" => {
                    if is_desc {
                        query.order_by_desc(academic_years::Column::CreatedAt)
                    } else {
                        query.order_by_asc(academic_years::Column::CreatedAt)
                    }
                }
                _ => query.order_by_desc(academic_years::Column::CreatedAt),
            };
        } else {
            query = query.order_by_desc(academic_years::Column::CreatedAt);
        }

        // Paginate
        let paginator = query.paginate(self.conn(), params.per_page());

        let total = paginator
            .num_items()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let items = paginator
            .fetch_page(params.page() - 1)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok((items, total))
    }

    /// Find active academic year for a foundation
    pub async fn find_active(
        &self,
        foundation_id: u64,
    ) -> Result<Option<academic_years::Model>, AppError> {
        AcademicYear::find()
            .filter(academic_years::Column::FoundationId.eq(foundation_id))
            .filter(academic_years::Column::IsActive.eq(1))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find by name within a foundation
    pub async fn find_by_name(
        &self,
        name: &str,
        foundation_id: u64,
    ) -> Result<Option<academic_years::Model>, AppError> {
        AcademicYear::find()
            .filter(academic_years::Column::FoundationId.eq(foundation_id))
            .filter(academic_years::Column::Name.eq(name))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Check if date range overlaps with existing academic years in same foundation
    pub async fn check_date_overlap(
        &self,
        start_date: &str,
        end_date: &str,
        foundation_id: u64,
        exclude_id: Option<u64>,
    ) -> Result<bool, AppError> {
        let mut query = AcademicYear::find()
            .filter(academic_years::Column::FoundationId.eq(foundation_id))
            .filter(academic_years::Column::StartDate.lte(end_date))
            .filter(academic_years::Column::EndDate.gte(start_date));

        if let Some(id) = exclude_id {
            query = query.filter(academic_years::Column::Id.ne(id));
        }

        let count = query
            .count(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(count > 0)
    }

    /// Update academic year
    pub async fn update(
        &self,
        id: u64,
        active_model: academic_years::ActiveModel,
    ) -> Result<academic_years::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);

        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Delete academic year
    pub async fn delete(&self, id: u64) -> Result<(), AppError> {
        AcademicYear::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Deactivate all academic years in a foundation
    pub async fn deactivate_all(&self, foundation_id: u64) -> Result<(), AppError> {
        use sea_orm::sea_query::Expr;

        AcademicYear::update_many()
            .filter(academic_years::Column::FoundationId.eq(foundation_id))
            .col_expr(academic_years::Column::IsActive, Expr::value(0))
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
