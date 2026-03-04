// ============================================================================
// backend/src/modules/regulations/repository.rs
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
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde_json::Value;

use super::dto::FoundationRegulationResponse;

#[derive(Clone)]
pub struct RegulationRepository {
    db: Database,
}

impl RegulationRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn conn(&self) -> &sea_orm::DatabaseConnection {
        self.db.get_connection()
    }

    // ========================================================================
    // MASTER REGULATION
    // ========================================================================

    pub async fn create(
        &self,
        active_model: regulations::ActiveModel,
    ) -> Result<regulations::Model, AppError> {
        active_model
            .insert(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<regulations::Model>, AppError> {
        Regulation::find_by_id(id)
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_by_code(&self, code: &str) -> Result<Option<regulations::Model>, AppError> {
        Regulation::find()
            .filter(regulations::Column::Code.eq(code))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn find_all(
        &self,
        params: &PaginationParams,
    ) -> Result<(Vec<regulations::Model>, u64), AppError> {
        let mut query = Regulation::find();

        if let Some(ref search) = params.search {
            query = query.filter(
                Condition::any()
                    .add(regulations::Column::Name.contains(search))
                    .add(regulations::Column::Code.contains(search)),
            );
        }

        query = match params.sort_by.as_deref() {
            Some("name") => match params.sort_order.as_deref() {
                Some("asc") => query.order_by_asc(regulations::Column::Name),
                _ => query.order_by_desc(regulations::Column::Name),
            },
            Some("code") => match params.sort_order.as_deref() {
                Some("asc") => query.order_by_asc(regulations::Column::Code),
                _ => query.order_by_desc(regulations::Column::Code),
            },
            _ => query.order_by_desc(regulations::Column::CreatedAt),
        };

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

    pub async fn update(
        &self,
        id: i64,
        active_model: regulations::ActiveModel,
    ) -> Result<regulations::Model, AppError> {
        let mut model = active_model;
        model.id = Set(id);
        model
            .update(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        Regulation::delete_by_id(id)
            .exec(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    pub async fn is_regulation_in_use(&self, regulation_id: i64) -> Result<bool, AppError> {
        let count = FoundationRegulation::find()
            .filter(foundation_regulations::Column::RegulationId.eq(regulation_id))
            .count(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(count > 0)
    }

    // ========================================================================
    // FOUNDATION
    // ========================================================================

    pub async fn find_foundation_by_code(
        &self,
        code: &str,
    ) -> Result<Option<foundations::Model>, AppError> {
        Foundation::find()
            .filter(foundations::Column::Code.eq(code))
            .one(self.conn())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    // ========================================================================
    // FOUNDATION REGULATION
    // ========================================================================

    pub async fn find_foundation_regulation(
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

    pub async fn find_all_with_foundation_status(
        &self,
        foundation_id: i64,
    ) -> Result<Vec<FoundationRegulationResponse>, AppError> {
        let rows = sea_orm::Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::MySql,
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
                let config: Option<Value> =
                    config_str.and_then(|s| serde_json::from_str(s.as_str()).ok());

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

    pub async fn upsert_foundation_regulation(
        &self,
        foundation_id: i64,
        regulation_id: i64,
        is_active: Option<i8>,
        config: Option<Value>,
    ) -> Result<FoundationRegulationResponse, AppError> {
        let existing = self
            .find_foundation_regulation(foundation_id, regulation_id)
            .await?;

        let regulation = self
            .find_by_id(regulation_id)
            .await?
            .ok_or_else(|| AppError::not_found("Regulation not found".to_string()))?;

        let model = match existing {
            Some(existing_model) => {
                let mut active = foundation_regulations::ActiveModel {
                    id: Set(existing_model.id),
                    is_active: Set(is_active),
                    updated_at: Set(chrono::Utc::now()),
                    ..Default::default()
                };
                if let Some(ref cfg) = config {
                    active.config = Set(Some(cfg.clone()));
                }
                active
                    .update(self.conn())
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?
            }
            None => {
                let active = foundation_regulations::ActiveModel {
                    foundation_id: Set(foundation_id),
                    regulation_id: Set(regulation_id),
                    is_active: Set(is_active),
                    config: Set(config),
                    created_at: Set(chrono::Utc::now()),
                    updated_at: Set(chrono::Utc::now()),
                    ..Default::default()
                };
                active
                    .insert(self.conn())
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?
            }
        };

        Ok(FoundationRegulationResponse {
            id: model.id,
            foundation_id: model.foundation_id,
            regulation_id: model.regulation_id,
            regulation_code: regulation.code,
            regulation_name: regulation.name,
            is_active: model.is_active.unwrap_or(0) != 0,
            config: model.config,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
            deleted_at: model.deleted_at.map(|d| d.to_string()),
        })
    }
}
