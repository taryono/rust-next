// backend/src/modules/user_profiles/dto.rs
use entity::sea_orm_active_enums::Gender; // ❌ Tidak punya ToSchema

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate; // Untuk validasi input // ✅ Built-in di Rust 1.80+
#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfileResponse {
    pub id: i64,
    pub foundation_id: i64,
    pub user_id: i64,
    pub phone: Option<String>,
    pub dob: Option<String>,
    pub pob: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<Gender>, // ❌ Error: Gender tidak implement ToSchema
    // Solusi 1: Tambahkan ToSchema ke Enum Gender (RECOMMENDED)
    // Buka file entity/src/sea_orm_active_enums.rs dan tambahkan derive ToSchema:
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserProfileRequest {
    pub foundation_id: i64,
    pub user_id: i64,
    #[validate(length(min = 10, max = 15, message = "Phone must be 10-15 characters"))]
    pub phone: Option<String>,
    pub dob: Option<String>,
    pub pob: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    #[validate(length(min = 6, max = 6))]
    pub postal_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserProfileRequest {
    pub user_id: i64,
    #[validate(length(min = 10, max = 15, message = "Phone must be 10-15 characters"))]
    pub phone: Option<String>,
    pub dob: Option<String>,
    pub pob: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    #[validate(length(min = 6, max = 6))]
    pub postal_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfileListResponse {
    pub data: Vec<UserProfileResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
// Implement From trait untuk convert entity ke response
impl From<entity::user_profiles::Model> for UserProfileResponse {
    fn from(model: entity::user_profiles::Model) -> Self {
        Self {
            id: model.id,
            foundation_id: model.foundation_id,
            user_id: model.user_id,
            phone: model.phone,
            dob: model.dob,
            pob: model.pob,
            bio: model.bio,
            avatar: model.avatar,
            gender: model.gender,
            address: model.address,
            city: model.city,
            province: model.province,
            country: model.country,
            postal_code: model.postal_code,
            latitude: model.latitude,
            longitude: model.longitude,
            timezone: model.timezone,
        }
    }
}
impl UserProfileResponse {
    #[warn(unused_imports)]
    pub fn from_entity(model: entity::user_profiles::Model) -> Self {
        Self::from(model)
    }
    #[warn(unused_imports)]
    pub fn from_vec(dto: Vec<entity::user_profiles::Model>) -> Vec<Self> {
        dto.into_iter().map(Self::from).collect()
    }
}

// // Regex untuk validasi phone
// static PHONE_REGEX: LazyLock<regex::Regex> =
//     LazyLock::new(|| regex::Regex::new(r"^\+?[0-9]{10,15}$").unwrap());
// // Regex untuk validasi postal code
// static POSTAL_CODE_REGEX: LazyLock<regex::Regex> =
//     LazyLock::new(|| regex::Regex::new(r"^[0-9]{6}$").unwrap());
