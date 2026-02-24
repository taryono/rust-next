// backend/src/modules/users/dto_multipart.rs
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Default)]
pub struct CreateUserMultipart {
    #[validate(length(min = 3))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 6))]
    pub password: String,

    pub dob: Option<String>,
    pub pob: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub bio: Option<String>,
    pub status: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,

    pub foundation_id: i64,

    #[serde(default = "default_is_active")]
    pub is_active: i8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
}

fn default_is_active() -> i8 {
    1 // default active
}

// Struct khusus untuk request multipart
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUserMultipartRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: Option<String>,
    pub pob: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub bio: Option<String>,
    pub status: String,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,
    pub foundation_id: String,
    pub roles: Option<Vec<String>>,
}
