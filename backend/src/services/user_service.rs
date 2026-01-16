use crate::{
    models::user::{
        ChangePasswordRequest, PaginationParams, UpdateUserRequest, UserListResponse, UserResponse,
    },
    utils::password,
};
use entity::role_users::{self as role_users, Entity as RoleUsers};
use entity::roles::{self as roles, Entity as Roles};
use entity::users::{self as users, Entity as User};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use std::collections::HashMap;
// ini reponse tanpa roles
// pub async fn get_all_users(
//     db: &DatabaseConnection,
// ) -> Result<UserListResponse, Box<dyn std::error::Error>> {
//     let users_list = User::find().all(db).await?;

//     let total = users_list.len();
//     let user_responses: Vec<UserResponse> = users_list
//         .iter()
//         .map(|u| UserResponse::from_entity(u))
//         .collect();

//     Ok(UserListResponse {
//         users: user_responses,
//         total,
//     })
// }

// ini response users beserta roles nya tanpa loader pattern
#[warn(dead_code)]
pub async fn get_all_users_old(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<UserListResponse, Box<dyn std::error::Error>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);
    // Query users dengan roles menggunakan find_with_related
    let users_with_roles = User::find().find_with_related(Roles).all(db).await?;

    let total = users_with_roles.len();
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u64;

    // Map ke UserResponse dengan roles
    let user_responses: Vec<UserResponse> = users_with_roles
        .into_iter()
        .map(|(user, roles)| UserResponse::from_user_with_roles(&user, &roles))
        .collect();

    Ok(UserListResponse {
        users: user_responses,
        total,
        page,
        per_page,
        total_pages,
    })
}

pub async fn get_all_users(
    db: &DatabaseConnection,
    params: PaginationParams,
) -> Result<UserListResponse, Box<dyn std::error::Error>> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(10);

    // Build query dengan filter
    let mut query = users::Entity::find();

    // Search filter
    if let Some(search_term) = &params.search {
        if !search_term.is_empty() {
            query = query.filter(
                Condition::any()
                    .add(users::Column::Name.contains(search_term))
                    .add(users::Column::Email.contains(search_term)),
            );
        }
    }

    // Order by created_at desc
    query = query.order_by_desc(users::Column::CreatedAt);

    // Get total count before pagination
    let total = query.clone().count(db).await? as usize;

    // Apply pagination
    let paginator = query.paginate(db, per_page);
    let users_list = paginator.fetch_page(page - 1).await?;

    // Calculate total pages
    let total_pages = ((total as f64) / (per_page as f64)).ceil() as u64;

    // If no users found, return early
    if users_list.is_empty() {
        return Ok(UserListResponse {
            users: vec![],
            total,
            page,
            per_page,
            total_pages,
        });
    }

    // Batch fetch roles - Collect user IDs
    let user_ids: Vec<u64> = users_list.iter().map(|u| u.id).collect();

    // Fetch all role_users with roles in one query
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

    // Optional: Filter by role if specified
    let users_list = if let Some(role_filter) = &params.role {
        users_list
            .into_iter()
            .filter(|user| {
                if let Some(roles) = user_roles_map.get(&user.id) {
                    roles
                        .iter()
                        .any(|r| r.name.eq_ignore_ascii_case(role_filter))
                } else {
                    false
                }
            })
            .collect()
    } else {
        users_list
    };

    // Map to UserResponse with roles
    let user_responses: Vec<UserResponse> = users_list
        .iter()
        .map(|user| {
            let roles = user_roles_map.get(&user.id).cloned().unwrap_or_default();
            UserResponse::from_user_with_roles(user, &roles)
        })
        .collect();

    Ok(UserListResponse {
        users: user_responses,
        total,
        page,
        per_page,
        total_pages,
    })
}

// function ini hanya mengembalikan data user tanpa roles
// pub async fn get_user_by_id(
//     db: &DatabaseConnection,
//     user_id: u64,
// ) -> Result<UserResponse, Box<dyn std::error::Error>> {
//     let user = User::find_by_id(user_id)
//         .one(db)
//         .await?
//         .ok_or("User not found")?;

//     Ok(UserResponse::from_entity(&user))
// }

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
    // let result = User::find_by_id(user_id)
    //     .find_with_related(Roles)
    //     .all(db)
    //     .await?;

    // let (user, roles) = result.into_iter().next().ok_or("User not found")?;

    // Ok(UserResponse::from_user_with_roles(&user, &roles))
}
// function ini hanya mengembalikan data user dengan roles namun dengan cara verbose / explisit
// pub async fn get_user_by_id(
//     db: &DatabaseConnection,
//     user_id: u64,
// ) -> Result<UserResponse, Box<dyn std::error::Error>> {
//     let (user, role_users) = User::find_by_id(user_id)
//         .find_with_related(RoleUsers)
//         .one(db)
//         .await?
//         .ok_or("User not found")?;

//     let roles = Roles::find()
//         .join(JoinType::InnerJoin, role_users::Relation::Role.def())
//         .filter(role_users::Column::UserId.eq(user.id))
//         .all(db)
//         .await?;

//     Ok(UserResponse::from_user_with_roles(&user, &roles))
// }

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
