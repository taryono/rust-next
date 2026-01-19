use crate::{
    modules::auth::models::{AuthResponse, LoginRequest, RefreshTokenResponse, RegisterRequest},
    modules::users::models::UserResponse,
    utils::{jwt, password},
};
use entity::roles::Entity as Roles;
use entity::users::{self as users, Entity as User};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::env;

pub async fn register_user(
    db: &DatabaseConnection,
    register_data: RegisterRequest,
) -> Result<AuthResponse, Box<dyn std::error::Error>> {
    // Check if email already exists
    let existing_user = User::find()
        .filter(users::Column::Email.eq(&register_data.email))
        .one(db)
        .await?;

    if existing_user.is_some() {
        return Err("Email already exists".into());
    }

    // Hash password
    let hashed_password = password::hash(&register_data.password)?;

    // Create new user
    let new_user = users::ActiveModel {
        name: Set(register_data.name),
        email: Set(register_data.email),
        password: Set(hashed_password),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    // Generate tokens
    let access_token = jwt::create_token(user.id.to_string())?;
    let refresh_token = jwt::create_refresh_token(user.id.to_string())?;

    let expires_in = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "900".to_string())
        .parse::<i64>()
        .unwrap_or(900);

    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
        user: UserResponse::from_entity(&user),
    })
}

pub async fn login_user(
    db: &DatabaseConnection,
    login_data: LoginRequest,
) -> Result<AuthResponse, Box<dyn std::error::Error>> {
    // Find user by email
    let user = User::find()
        .filter(users::Column::Email.eq(&login_data.email))
        .one(db)
        .await?
        .ok_or("Invalid email or password")?;

    // Verify password
    if !password::verify(&login_data.password, &user.password)? {
        return Err("Invalid email or password".into());
    }

    // Generate tokens
    let access_token = jwt::create_token(user.id.to_string())?;
    let refresh_token = jwt::create_refresh_token(user.id.to_string())?;

    let expires_in = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "900".to_string())
        .parse::<i64>()
        .unwrap_or(900);

    // Query roles user
    let (user, roles) = User::find_by_id(user.id)
        .find_with_related(Roles)
        .all(db)
        .await?
        .into_iter()
        .next()
        .ok_or("User not found")?;
    println!("Roles: {:?}", roles);
    // atau
    dbg!(&roles);
    Ok(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
        user: UserResponse::from_user_with_roles(&user, &roles),
    })
}

pub async fn refresh_token(
    refresh_token: String,
) -> Result<RefreshTokenResponse, Box<dyn std::error::Error>> {
    // Verify refresh token
    let claims = jwt::verify_refresh_token(&refresh_token)?;

    // Generate new tokens
    let new_access_token = jwt::create_token(claims.sub.clone())?;
    let new_refresh_token = jwt::create_refresh_token(claims.sub)?;

    let expires_in = env::var("JWT_EXPIRATION")
        .unwrap_or_else(|_| "900".to_string())
        .parse::<i64>()
        .unwrap_or(900);

    Ok(RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
    })
}
