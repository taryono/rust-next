// ============================================================================
// repository.rs - Database Operations Only
// api/src/modules/foundation_regulations/repository.rs
// ============================================================================
use crate::config::database::Database;
use crate::errors::AppError;
use crate::utils::pagination::PaginationParams;
use entity::{
    foundation_regulations::{self, Entity as FoundationRegulation},
    foundations::{self, Entity as Foundation},
    regulations::{self, Entity as Regulation},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde_json::Value;

use super::dto::FoundationRegulationResponse;

#[derive(Clone)]
pub struct FoundationRegulationRepository {
    db: Database,
}

impl FoundationRegulationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    // ========================================================================
    // FOUNDATION LOOKUP
    // ========================================================================

    /// Find foundation by code
    pub async fn find_foundation_by_code(
        &self,
        code: &str,
    ) -> Result<Option<foundations::Model>, AppError> {
        Foundation::find()
            .filter(foundations::Column::Code.eq(code))
            .filter(foundations::Column::DeletedAt.is_null())
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find regulation by code
    pub async fn find_regulation_by_code(
        &self,
        code: &str,
    ) -> Result<Option<regulations::Model>, AppError> {
        Regulation::find()
            .filter(regulations::Column::Code.eq(code))
            .filter(regulations::Column::DeletedAt.is_null())
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    // ========================================================================
    // CRUD
    // ========================================================================

    /// Find by ID (exclude soft deleted)
    pub async fn find_by_id(
        &self,
        id: i64,
    ) -> Result<Option<foundation_regulations::Model>, AppError> {
        FoundationRegulation::find_by_id(id)
            .filter(foundation_regulations::Column::DeletedAt.is_null())
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Find satu record berdasarkan foundation_id + regulation_id
    pub async fn find_by_foundation_and_regulation(
        &self,
        foundation_id: i64,
        regulation_id: i64,
    ) -> Result<Option<foundation_regulations::Model>, AppError> {
        FoundationRegulation::find()
            .filter(foundation_regulations::Column::FoundationId.eq(foundation_id))
            .filter(foundation_regulations::Column::RegulationId.eq(regulation_id))
            .filter(foundation_regulations::Column::DeletedAt.is_null())
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// Get semua regulasi + status aktif untuk satu yayasan (LEFT JOIN)
    /// Regulasi yang belum diassign pun tetap muncul dengan is_active = false
    pub async fn find_all_with_foundation_status(
        &self,
        foundation_id: i64,
    ) -> Result<Vec<FoundationRegulationResponse>, AppError> {
        let rows = sea_orm::Statement::from_sql_and_values(
            sea_orm::Databaseapi::MySql,
            r#"
                SELECT
                    COALESCE(fr.id, 0)                    AS id,
                    r.id                                   AS regulation_id,
                    ?                                      AS foundation_id,
                    r.code                                 AS regulation_code,
                    r.name                                 AS regulation_name,
                    COALESCE(fr.is_active, 0)             AS is_active,
                    fr.config                              AS config,
                    fr.deleted_at                          AS deleted_at,
                    COALESCE(fr.created_at, r.created_at) AS created_at,
                    COALESCE(fr.updated_at, r.updated_at) AS updated_at
                FROM regulations r
                LEFT JOIN foundation_regulations fr
                    ON fr.regulation_id = r.id
                    AND fr.foundation_id = ?
                    AND fr.deleted_at IS NULL
                WHERE r.deleted_at IS NULL
                ORDER BY r.code ASC
            "#,
            vec![
                sea_orm::Value::BigInt(Some(foundation_id)),
                sea_orm::Value::BigInt(Some(foundation_id)),
            ],
        );

        let results = self
            .conn()
            .query_all_raw(rows)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let responses = results
            .iter()
            .map(|row| {
                let config_str: Option<String> = row.try_get("", "config").ok().flatten();
                let config: Option<Value> = config_str.and_then(|s| serde_json::from_str(&s).ok());

                FoundationRegulationResponse {
                    id: row.try_get("", "id").unwrap_or(0),
                    foundation_id: row.try_get("", "foundation_id").unwrap_or(0),
                    regulation_id: row.try_get("", "regulation_id").unwrap_or(0),
                    regulation_code: row.try_get("", "regulation_code").unwrap_or_default(),
                    regulation_name: row.try_get("", "regulation_name").unwrap_or_default(),
                    is_active: row.try_get::<i8>("", "is_active").unwrap_or(0) != 0,
                    config,
                    created_at: row
                        .try_get::<chrono::DateTime<chrono::Utc>>("", "created_at")
                        .map(|d| d.to_string())
                        .unwrap_or_default(),
                    updated_at: row
                        .try_get::<chrono::DateTime<chrono::Utc>>("", "updated_at")
                        .map(|d| d.to_string())
                        .unwrap_or_default(),
                    deleted_at: row
                        .try_get::<chrono::DateTime<chrono::Utc>>("", "deleted_at")
                        .ok()
                        .map(|d| d.to_string()),
                }
            })
            .collect();

        Ok(responses)
    }

    /// Find all dengan pagination (untuk kebutuhan admin)
    pub async fn find_all(
        &self,
        params: &PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<(Vec<foundation_regulations::Model>, u64), AppError> {
        let mut query = FoundationRegulation::find()
            .filter(foundation_regulations::Column::DeletedAt.is_null());

        // Filter by foundation jika ada
        if let Some(fid) = foundation_id {
            query = query.filter(foundation_regulations::Column::FoundationId.eq(fid));
        }

        query = query.order_by_desc(foundation_regulations::Column::UpdatedAt);

        let per_page = params.per_page() as u64;
        let paginator = query.paginate(self.conn(), per_page);

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

    /// Upsert: insert jika belum ada, update jika sudah ada
    pub async fn upsert(
        &self,
        foundation_id: i64,
        regulation_id: i64,
        is_active: bool,
        config: Option<Value>,
    ) -> Result<foundation_regulations::Model, AppError> {
        let existing = self
            .find_by_foundation_and_regulation(foundation_id, regulation_id)
            .await?;

        match existing {
            Some(model) => {
                // Update existing
                let mut active = foundation_regulations::ActiveModel {
                    id: Set(model.id),
                    is_active: Set(Some(is_active as i8)),
                    updated_at: Set(chrono::Utc::now()),
                    ..Default::default()
                };
                if let Some(cfg) = config {
                    active.config = Set(Some(cfg));
                }
                active
                    .update(self.conn())
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))
            }
            None => {
                // Insert baru
                let active = foundation_regulations::ActiveModel {
                    foundation_id: Set(foundation_id),
                    regulation_id: Set(regulation_id),
                    is_active: Set(Some(is_active as i8)),
                    config: Set(config),
                    created_at: Set(chrono::Utc::now()),
                    updated_at: Set(chrono::Utc::now()),
                    ..Default::default()
                };
                active
                    .insert(self.conn())
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))
            }
        }
    }

    /// Soft delete
    pub async fn soft_delete(&self, id: i64) -> Result<(), AppError> {
        let active = foundation_regulations::ActiveModel {
            id: Set(id),
            deleted_at: Set(Some(chrono::Utc::now())),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };
        active
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
