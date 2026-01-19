// ============================================
// backend/src/services/role_service.rs
// ============================================
use crate::modules::roles::models::{RoleResponse, UpdateRoleRequest};
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use chrono::Utc;
use entity::roles::{self, Entity as Roles};
use entity::traits::soft_delete::SoftDelete;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

pub async fn get_all_roles(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<RoleResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Build base query - hanya yang tidak dihapus
    let mut query = Roles::find_not_deleted();

    // Apply search filter
    if let Some(search_term) = &params.search {
        if !search_term.is_empty() {
            query = query.filter(
                Condition::any()
                    .add(roles::Column::Name.contains(search_term))
                    .add(roles::Column::Description.contains(search_term)),
            );
        }
    }

    // Apply sorting
    query = match params.sort_order.as_deref() {
        Some("asc") => query.order_by_asc(roles::Column::CreatedAt),
        _ => query.order_by_desc(roles::Column::CreatedAt),
    };

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let roles_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let role_responses: Vec<RoleResponse> = roles_list
        .iter()
        .map(|role| RoleResponse::from_entity(role))
        .collect();

    Ok(PaginatedResponse::new(
        role_responses,
        total,
        page,
        per_page,
    ))
}

pub async fn get_role_by_id(db: &DatabaseConnection, role_id: u64) -> Result<RoleResponse, DbErr> {
    let role = Roles::find_not_deleted()
        .filter(roles::Column::Id.eq(role_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Role not found".to_string()))?;

    Ok(RoleResponse::from_entity(&role))
}

pub async fn update_role(
    db: &DatabaseConnection,
    role_id: u64,
    update_data: UpdateRoleRequest,
) -> Result<RoleResponse, DbErr> {
    let role = Roles::find_not_deleted()
        .filter(roles::Column::Id.eq(role_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Role not found".to_string()))?;

    let mut active_role: roles::ActiveModel = role.into_active_model();

    // ✅ Langsung set tanpa if let karena name bukan Option
    active_role.name = Set(update_data.name);

    // Description tetap optional
    if let Some(description) = update_data.description {
        // Check if description already exists
        let existing = Roles::find()
            .filter(roles::Column::Description.eq(&description))
            .one(db)
            .await?;

        if let Some(existing_role) = existing {
            if existing_role.id != role_id {
                return Err(DbErr::Custom("Description already in use".to_string()));
            }
        }
        active_role.description = Set(Some(description));
    }

    active_role.updated_at = Set(Utc::now());

    let updated_role = active_role.update(db).await?;

    Ok(RoleResponse::from_entity(&updated_role))
}

/// Soft delete role
pub async fn soft_delete(db: &DatabaseConnection, role_id: u64) -> Result<(), DbErr> {
    let role = Roles::find_by_id(role_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Role not found".to_string()))?;

    let mut active_role: roles::ActiveModel = role.into_active_model();
    active_role.deleted_at = Set(Some(Utc::now()));
    active_role.update(db).await?;

    Ok(())
}

/// Restore deleted role
pub async fn restore(db: &DatabaseConnection, role_id: u64) -> Result<(), DbErr> {
    let role = Roles::find_by_id(role_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("Role not found".to_string()))?;

    let mut active_role: roles::ActiveModel = role.into_active_model();
    active_role.deleted_at = Set(None);
    active_role.update(db).await?;

    Ok(())
}

/// Get deleted roles (with pagination)
pub async fn get_deleted_roles(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<RoleResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query hanya yang sudah dihapus
    let query = Roles::find_only_deleted();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let roles_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let role_responses: Vec<RoleResponse> = roles_list
        .iter()
        .map(|role| RoleResponse::from_entity(role))
        .collect();

    Ok(PaginatedResponse::new(
        role_responses,
        total,
        page,
        per_page,
    ))
}

/// Get all roles including deleted
pub async fn get_all_with_deleted(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<RoleResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query semua termasuk yang dihapus
    let query = Roles::find_with_deleted();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let roles_list = paginator.fetch_page(page - 1).await?;

    // Map to response
    let role_responses: Vec<RoleResponse> = roles_list
        .iter()
        .map(|role| RoleResponse::from_entity(role))
        .collect();

    Ok(PaginatedResponse::new(
        role_responses,
        total,
        page,
        per_page,
    ))
}

/// Force delete (permanent)
pub async fn force_delete(db: &DatabaseConnection, role_id: u64) -> Result<(), DbErr> {
    Roles::delete_by_id(role_id).exec(db).await?;
    Ok(())
}
