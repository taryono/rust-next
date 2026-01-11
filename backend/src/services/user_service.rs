use crate::{
    models::user::{ChangePasswordRequest, UpdateUserRequest, UserListResponse, UserResponse},
    utils::password,
};
use entity::users::{self as users, Entity as User};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};

pub async fn get_all_users(db: &DatabaseConnection) -> Result<UserListResponse, Box<dyn std::error::Error>> {
    let users_list = User::find()
        .all(db)
        .await?;

    let total = users_list.len();
    let user_responses: Vec<UserResponse> = users_list
        .iter()
        .map(|u| UserResponse::from_entity(u))
        .collect();

    Ok(UserListResponse {
        users: user_responses,
        total,
    })
}

pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<UserResponse, Box<dyn std::error::Error>> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or("User not found")?;

    Ok(UserResponse::from_entity(&user))
}

pub async fn update_user(
    db: &DatabaseConnection,
    user_id: i32,
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
    user_id: i32,
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
    user_id: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = User::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or("User not found")?;

    let active_user: users::ActiveModel = user.into();
    active_user.delete(db).await?;

    Ok(())
}