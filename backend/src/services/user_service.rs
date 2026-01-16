use crate::models::pagination::{PaginatedResponse, PaginationParams};
use crate::{
    models::user::{ChangePasswordRequest, UpdateUserRequest, UserResponse},
    utils::password,
};
use entity::role_users::{self as role_users};
use entity::roles::{self as roles, Entity as Roles};
use entity::users::{self as users, Entity as User};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use std::collections::HashMap;
pub async fn get_all_users(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<PaginatedResponse<UserResponse>, Box<dyn std::error::Error>> {
    let page = params.page();
    let per_page = params.per_page();

    // Build base query
    let mut query = users::Entity::find();

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
    let total = query.clone().count(db).await? as usize;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let users_list = paginator.fetch_page(page - 1).await?;

    // Early return if no users
    if users_list.is_empty() {
        return Ok(PaginatedResponse::new(vec![], total, page, per_page));
    }

    // Batch load roles
    let user_ids: Vec<u64> = users_list.iter().map(|u| u.id).collect();
    let role_users_with_roles = role_users::Entity::find()
        .filter(role_users::Column::UserId.is_in(user_ids))
        .find_also_related(roles::Entity)
        .all(db)
        .await?;

    // Group roles by user_id
    let mut user_roles_map: HashMap<u64, Vec<roles::Model>> = HashMap::new();
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

// function ini hanya mengembalikan data user dengan roles namun dengan cara magic
pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: u64,
) -> Result<UserResponse, Box<dyn std::error::Error>> {
    let (user, roles) = User::find_by_id(user_id)
        .find_with_related(Roles)
        .all(db)
        .await?
        .into_iter()
        .next()
        .ok_or("User not found")?;
    println!("Roles: {:?}", roles);
    // atau
    dbg!(&roles);
    Ok(UserResponse::from_user_with_roles(&user, &roles))
}
pub async fn update_user(
    db: &DatabaseConnection,
    user_id: u64,
    update_data: UpdateUserRequest,
) -> Result<UserResponse, Box<dyn std::error::Error>> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or("User not found")?;

    let mut active_user: users::ActiveModel = user.into();

    if let Some(name) = update_data.name {
        active_user.name = Set(name);
    }

    if let Some(ref email) = update_data.email {
        // Check if email already exists
        let existing = User::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await?;

        if let Some(existing_user) = existing {
            if existing_user.id != user_id {
                return Err("Email already in use".into());
            }
        }

        active_user.email = Set(email.clone());
    }

    let updated_user = active_user.update(db).await?;

    Ok(UserResponse::from_entity(&updated_user))
}

pub async fn change_password(
    db: &DatabaseConnection,
    user_id: u64,
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

pub async fn delete_user(
    db: &DatabaseConnection,
    user_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or("User not found")?;

    let active_user: users::ActiveModel = user.into();
    active_user.delete(db).await?;

    Ok(())
}
