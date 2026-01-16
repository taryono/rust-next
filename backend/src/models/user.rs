use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,

    #[validate(email)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1))]
    pub old_password: String,

    #[validate(length(min = 6))]
    pub new_password: String,
}

// Untuk konversi dari Entity ke Response DTO
impl UserResponse {
    pub fn from_entity(user: &entity::users::Model) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            created_at: user.created_at.as_ref().map(|dt| dt.to_string()),
            updated_at: user.updated_at.as_ref().map(|dt| dt.to_string()),
            roles: user
                .role_users
                .into_iter()
                .map(|(_, role)| role.unwrap().name)
                .collect(),
        }
    }
}
