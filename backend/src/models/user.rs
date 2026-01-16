use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: usize,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PaginationParams {
    #[validate(range(min = 1))]
    pub page: Option<u64>,

    #[validate(range(min = 1, max = 100))]
    pub per_page: Option<u64>,

    pub search: Option<String>,
    pub role: Option<String>,
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
impl UserResponse {
    pub fn from_user_with_roles(
        user: &entity::users::Model,
        roles: &[entity::roles::Model],
    ) -> Self {
        let role_names: Vec<String> = roles.iter().map(|r| r.name.clone()).collect();
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            created_at: user.created_at.map(|d| d.to_string()),
            updated_at: user.updated_at.map(|d| d.to_string()),
            roles: Some(role_names),
        }
    }

    pub fn from_entity(user: &entity::users::Model) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
            created_at: user.created_at.as_ref().map(|dt| dt.to_string()),
            updated_at: user.updated_at.as_ref().map(|dt| dt.to_string()),
            roles: None,
        }
    }
}
