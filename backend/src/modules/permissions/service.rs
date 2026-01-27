// ============================================================================
// backend/src/modules/permissions/service.rs
// service.rs - Business Logic Only
// ============================================================================
use super::dto::{CreatePermissionRequest, PermissionResponse, UpdatePermissionRequest};
use super::repository::PermissionRepository;
use crate::errors::AppError;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use entity::{permissions, role_permissions, role_users, user_permissions};
use sea_orm::prelude::Expr;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, RelationTrait,
};
use sea_orm::{FromQueryResult, QuerySelect, Set};
use std::collections::HashSet;
use validator::Validate;
#[derive(Clone)]
pub struct PermissionService {
    repository: PermissionRepository,
}

impl PermissionService {
    pub fn new(repository: PermissionRepository) -> Self {
        Self { repository }
    }

    /// Create new permission with validation
    pub async fn create(
        &self,
        request: CreatePermissionRequest,
    ) -> Result<PermissionResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check duplicate name
        if let Some(_) = self
            .repository
            .find_by_name(&request.name, request.foundation_id)
            .await?
        {
            return Err(AppError::ConflictError(
                "Permission with this name already exists".to_string(),
            ));
        }

        // Parse start_date and end_date to NaiveDate
        // Build entity with parsed dates
        let active_model = permissions::ActiveModel {
            foundation_id: Set(request.foundation_id),
            code: Set(request.code),
            name: Set(request.name),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        // Delegate to repository
        let created = self.repository.create(active_model).await?;

        // Convert to response (Date → String otomatis lewat From trait)
        Ok(PermissionResponse::from(created))
    }

    /// Get permission by ID
    pub async fn get_by_id(&self, id: i64) -> Result<PermissionResponse, AppError> {
        let permission = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Permission not found".to_string()))?;

        Ok(PermissionResponse::from(permission))
    }

    /// Get all permissions with pagination
    pub async fn get_all(
        &self,
        params: PaginationParams,
        foundation_id: Option<i64>,
    ) -> Result<PaginatedResponse<PermissionResponse>, AppError> {
        // Validate pagination params
        params
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        let (items, total) = self.repository.find_all(&params, foundation_id).await?;

        let responses: Vec<PermissionResponse> =
            items.into_iter().map(PermissionResponse::from).collect();

        Ok(PaginatedResponse::new(
            responses,
            total,
            params.page(),
            params.per_page(),
        ))
    }

    /// Update permission
    pub async fn update(
        &self,
        id: i64,
        request: UpdatePermissionRequest,
    ) -> Result<PermissionResponse, AppError> {
        // Validate request
        request
            .validate()
            .map_err(|e| AppError::validation(e.to_string()))?;

        // Check if exists
        let existing = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Permission not found".to_string()))?;

        // Business rule: check duplicate name if changing
        if let Some(ref name) = request.name {
            if name != &existing.name {
                if let Some(_) = self
                    .repository
                    .find_by_name(name, existing.foundation_id)
                    .await?
                {
                    return Err(AppError::ConflictError(
                        "Permission with this name already exists".to_string(),
                    ));
                }
            }
        }
        // Build update model
        let mut active_model = permissions::ActiveModel {
            id: Set(id),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        if let Some(name) = request.name {
            active_model.name = Set(name);
        }

        // Delegate to repository
        let updated = self.repository.update(id, active_model).await?;

        Ok(PermissionResponse::from(updated))
    }

    /// Delete permission
    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        // Check if exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::not_found("Permission not found".to_string()))?;

        // Business rule: Add any deletion constraints here
        // e.g., cannot delete if has related semesters
        // You can add repository method to check relations

        self.repository.delete(id).await
    }
    pub async fn resolve_user_permissions(
        db: &DatabaseConnection,
        user_id: i64,
        foundation_id: i64,
    ) -> Result<HashSet<String>, AppError> {
        let mut permissions = HashSet::new();

        permissions.extend(Self::get_permissions_from_roles(db, user_id, foundation_id).await?);

        permissions.extend(Self::get_permissions_from_user(db, user_id, foundation_id).await?);
        dbg!(permissions.iter().clone());
        Ok(permissions)
    }

    /// Get permissions via roles
    // async fn get_permissions_from_roles(
    //     db: &DatabaseConnection,
    //     user_id: i64,
    // ) -> Result<Vec<String>, AppError> {
    //     let rows = permissions::Entity::find()
    //         // 1. Join ke role_permissions
    //         .join_rev(
    //             sea_orm::JoinType::InnerJoin,
    //             role_permissions::Relation::Permissions.def(),
    //         )
    //         // 2. Join ke role_users secara manual pada kolom role_id
    //         // Ini menghindari error "Unknown column roles.id" karena kita tidak memanggil tabel roles
    //         .join(
    //             sea_orm::JoinType::InnerJoin,
    //             role_permissions::Entity::belongs_to(role_users::Entity)
    //                 .from(role_permissions::Column::RoleId)
    //                 .to(role_users::Column::RoleId)
    //                 .into(),
    //         )
    //         .filter(role_users::Column::UserId.eq(user_id))
    //         .all(db)
    //         .await?
    //         .build(DbBackend::MySql);

    //     // Cetak query ke terminal (mirip dd() tapi hanya untuk SQL string)
    //     println!("DEBUG SQL: {}", rows.to_string());

    //     Ok(rows.into_iter().map(|p| p.code).collect())
    // }

    async fn get_permissions_from_roles(
        db: &DatabaseConnection,
        user_id: i64,
        foundation_id: i64,
    ) -> Result<Vec<String>, AppError> {
        #[derive(FromQueryResult)]
        struct PermCode {
            code: String,
        }

        // Gunakan sea_query untuk build query dengan benar
        let mut query = sea_orm::sea_query::Query::select();

        query
            .distinct()
            .column((permissions::Entity, permissions::Column::Code))
            .from(permissions::Entity)
            .inner_join(
                role_permissions::Entity,
                Expr::col((permissions::Entity, permissions::Column::Id)).equals((
                    role_permissions::Entity,
                    role_permissions::Column::PermissionId,
                )),
            )
            .inner_join(
                role_users::Entity,
                Expr::col((role_permissions::Entity, role_permissions::Column::RoleId))
                    .equals((role_users::Entity, role_users::Column::RoleId)),
            )
            .and_where(Expr::col((role_users::Entity, role_users::Column::UserId)).eq(user_id))
            .and_where(Expr::col((permissions::Entity, permissions::Column::DeletedAt)).is_null())
            .and_where(
                Expr::col((
                    role_permissions::Entity,
                    role_permissions::Column::DeletedAt,
                ))
                .is_null(),
            )
            .and_where(Expr::col((role_users::Entity, role_users::Column::DeletedAt)).is_null())
            .and_where(Expr::col((
                role_permissions::Entity,
                role_permissions::Column::FoundationId,
            )))
            .and_where(
                Expr::col((permissions::Entity, permissions::Column::FoundationId))
                    .eq(foundation_id),
            );
        let builder = db.get_database_backend();
        let statement = builder.build(&query);

        println!("DEBUG SQL: {}", statement.to_string());

        let results = PermCode::find_by_statement(statement).all(db).await?;

        Ok(results.into_iter().map(|r| r.code).collect())
    }

    /// Get direct permissions from user_permissions
    async fn get_permissions_from_user(
        db: &DatabaseConnection,
        user_id: i64,
        foundation_id: i64,
    ) -> Result<Vec<String>, AppError> {
        let rows = permissions::Entity::find()
            // Gunakan join_rev untuk menyambung dari permissions ke user_permissions
            .join_rev(
                sea_orm::JoinType::InnerJoin,
                user_permissions::Relation::Permissions.def(),
            )
            .filter(user_permissions::Column::UserId.eq(user_id))
            .filter(permissions::Column::FoundationId.eq(foundation_id))
            .all(db)
            .await?;

        Ok(rows.into_iter().map(|p| p.code).collect())
    }
}
