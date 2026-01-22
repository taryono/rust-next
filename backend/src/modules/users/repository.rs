// ============================================
// backend/src/service/user_service.rs
// ============================================
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use crate::{
    modules::users::dto::{ChangePasswordRequest, UpdateUserRequest, UserResponse},
    utils::password,
};
use chrono::Utc;
use entity::role_users;
use entity::roles::{self, Entity as Roles};
use entity::traits::soft_delete::SoftDelete;
use entity::users::{self, Entity as User};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use std::collections::HashMap;

pub async fn get_all_users(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<UserResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Build base query - hanya yang tidak dihapus
    let mut query = User::find_not_deleted();

    // Apply search filter
    if let Some(search_term) = &params.search {
        if !search_term.is_empty() {
            query = query.filter(
                Condition::any()
                    .add(users::Column::Name.contains(search_term))
                    .add(users::Column::Email.contains(search_term)),
            );
        }
    }

    // Apply sorting
    query = match params.sort_order.as_deref() {
        Some("asc") => query.order_by_asc(users::Column::CreatedAt),
        _ => query.order_by_desc(users::Column::CreatedAt),
    };

    // Get total count
    // let total = query.clone().count(db).await? as i64;
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let users_list = paginator.fetch_page(page - 1).await?;

    // Early return if no users
    if users_list.is_empty() {
        return Ok(PaginatedResponse::new(vec![], total, page, per_page));
    }

    // Batch load roles
    let user_ids: Vec<i64> = users_list.iter().map(|u| u.id).collect();
    let role_users_with_roles = role_users::Entity::find()
        .filter(role_users::Column::UserId.is_in(user_ids))
        .find_also_related(roles::Entity)
        .all(db)
        .await?;

    // Group roles by user_id
    let mut user_roles_map: HashMap<i64, Vec<roles::Model>> = HashMap::new();
    for (role_user, role_opt) in role_users_with_roles {
        if let Some(role) = role_opt {
            user_roles_map
                .entry(role_user.user_id)
                .or_insert_with(Vec::new)
                .push(role);
        }
    }

    // Map to response
    let user_responses: Vec<UserResponse> = users_list
        .iter()
        .map(|user| {
            let roles = user_roles_map.get(&user.id).cloned().unwrap_or_default();
            UserResponse::from_user_with_roles(user, &roles)
        })
        .collect();

    Ok(PaginatedResponse::new(
        user_responses,
        total,
        page,
        per_page,
    ))
}

pub async fn get_by_id(db: &DatabaseConnection, user_id: i64) -> Result<UserResponse, DbErr> {
    let (user, roles) = User::find_not_deleted()
        .filter(users::Column::Id.eq(user_id))
        .find_with_related(Roles)
        .all(db)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_string()))?;

    Ok(UserResponse::from_user_with_roles(&user, &roles))
}

pub async fn update_user(
    db: &DatabaseConnection,
    user_id: i64,
    update_data: UpdateUserRequest,
) -> Result<UserResponse, DbErr> {
    let user = User::find_not_deleted()
        .filter(users::Column::Id.eq(user_id))
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_string()))?;

    let mut active_user: users::ActiveModel = user.into_active_model();

    if let Some(name) = update_data.name {
        active_user.name = Set(name);
    }

    if let Some(email) = update_data.email {
        // Check if email already exists
        let existing = User::find()
            .filter(users::Column::Email.eq(&email))
            .one(db)
            .await?;

        if let Some(existing_user) = existing {
            if existing_user.id != user_id {
                return Err(DbErr::Custom("Email already in use".to_string()));
            }
        }
        active_user.email = Set(email);
    }

    active_user.updated_at = Set(Utc::now());

    let updated_user = active_user.update(db).await?;

    Ok(UserResponse::from_entity(&updated_user))
}

pub async fn change_password(
    db: &DatabaseConnection,
    user_id: i64,
    password_data: ChangePasswordRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or("User not found")?;

    // Verify old password
    if !password::verify(&password_data.old_password, &user.password)? {
        return Err("Incorrect old password".into());
    }

    // Hash new password
    let hashed_password = password::hash(&password_data.new_password)?;

    let mut active_user: users::ActiveModel = user.into();
    active_user.password = Set(hashed_password);

    active_user.update(db).await?;

    Ok(())
}

// pub async fn change_password(
//     db: &DatabaseConnection,
//     user_id: i64,
//     password_data: ChangePasswordRequest,
// ) -> Result<(), DbErr> {
//     let user = User::find_not_deleted()
//         .filter(users::Column::Id.eq(user_id))
//         .one(db)
//         .await?
//         .ok_or_else(|| DbErr::RecordNotFound("User not found".to_string()))?;

//     // Verify old password
//     if !password::verify(&password_data.old_password, &user.password)? {
//         return Err(DbErr::Custom("Incorrect old password".to_string()));
//     }

//     // Hash new password
//     let hashed_password = password::hash(&password_data.new_password)?;

//     let mut active_user: users::ActiveModel = user.into_active_model();
//     active_user.password = Set(hashed_password);

//     active_user.update(db).await?;

//     Ok(())
// }

/// Soft delete user
pub async fn soft_delete(db: &DatabaseConnection, user_id: i64) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_string()))?;

    let mut active_user: users::ActiveModel = user.into_active_model();
    active_user.deleted_at = Set(Some(Utc::now()));
    active_user.update(db).await?;

    Ok(())
}

/// Restore deleted user
pub async fn restore(db: &DatabaseConnection, user_id: i64) -> Result<(), DbErr> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("User not found".to_string()))?;

    let mut active_user: users::ActiveModel = user.into_active_model();
    active_user.deleted_at = Set(None);
    active_user.update(db).await?;

    Ok(())
}

/// Get deleted users (with pagination)
pub async fn get_deleted_users(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<UserResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query hanya yang sudah dihapus
    let query = User::find_only_deleted();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let users_list = paginator.fetch_page(page - 1).await?;

    // Early return if no users
    if users_list.is_empty() {
        return Ok(PaginatedResponse::new(vec![], total, page, per_page));
    }

    // Batch load roles (sama seperti get_all_users)
    let user_ids: Vec<i64> = users_list.iter().map(|u| u.id).collect();
    let role_users_with_roles = role_users::Entity::find()
        .filter(role_users::Column::UserId.is_in(user_ids))
        .find_also_related(roles::Entity)
        .all(db)
        .await?;

    let mut user_roles_map: HashMap<i64, Vec<roles::Model>> = HashMap::new();
    for (role_user, role_opt) in role_users_with_roles {
        if let Some(role) = role_opt {
            user_roles_map
                .entry(role_user.user_id)
                .or_insert_with(Vec::new)
                .push(role);
        }
    }

    // Map to response
    let user_responses: Vec<UserResponse> = users_list
        .iter()
        .map(|user| {
            let roles = user_roles_map.get(&user.id).cloned().unwrap_or_default();
            UserResponse::from_user_with_roles(user, &roles)
        })
        .collect();

    Ok(PaginatedResponse::new(
        user_responses,
        total,
        page,
        per_page,
    ))
}

/// Get all users including deleted
pub async fn get_all_with_deleted(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<UserResponse>, DbErr> {
    let page = params.page();
    let per_page = params.per_page();

    // Query semua termasuk yang dihapus
    let query = User::find_with_deleted();

    // Get total count
    let paginator = query.clone().paginate(db, per_page);
    let total = paginator.num_items().await?;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let users_list = paginator.fetch_page(page - 1).await?;

    // Map to response (sama seperti sebelumnya)
    let user_ids: Vec<i64> = users_list.iter().map(|u| u.id).collect();
    let role_users_with_roles = role_users::Entity::find()
        .filter(role_users::Column::UserId.is_in(user_ids))
        .find_also_related(roles::Entity)
        .all(db)
        .await?;

    let mut user_roles_map: HashMap<i64, Vec<roles::Model>> = HashMap::new();
    for (role_user, role_opt) in role_users_with_roles {
        if let Some(role) = role_opt {
            user_roles_map
                .entry(role_user.user_id)
                .or_insert_with(Vec::new)
                .push(role);
        }
    }

    let user_responses: Vec<UserResponse> = users_list
        .iter()
        .map(|user| {
            let roles = user_roles_map.get(&user.id).cloned().unwrap_or_default();
            UserResponse::from_user_with_roles(user, &roles)
        })
        .collect();

    Ok(PaginatedResponse::new(
        user_responses,
        total,
        page,
        per_page,
    ))
}

/// Force delete (permanent)
pub async fn force_delete(db: &DatabaseConnection, user_id: i64) -> Result<(), DbErr> {
    User::delete_by_id(user_id).exec(db).await?;
    Ok(())
}
