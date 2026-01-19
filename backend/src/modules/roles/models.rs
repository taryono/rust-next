use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: u64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleListResponse {
    pub roles: Vec<RoleResponse>,
    pub total: usize,
    pub page: u64,
    pub per_page: u64,
    pub total_pages: u64,
}
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    pub description: Option<String>,
}
impl RoleResponse {
    pub fn from_entity(role: &entity::roles::Model) -> Self {
        Self {
            id: role.id,
            name: role.name.clone(),
            description: role.description.clone(),
            created_at: role.created_at.to_string(),
            //  Dipakai jika updated_at adalah Option (tipe: Option<DateTime>)
            // role.updated_at.as_ref().map(|dt| dt.to_string()),
            // Dipakai jika updated_at bukan Option (tipe: DateTime)
            updated_at: role.updated_at.to_string(),
        }
    }
}
