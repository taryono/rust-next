use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationResponse {
    pub id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FoundationListResponse {
    pub foundations: Vec<FoundationResponse>,
    pub total: usize,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateFoundationRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    pub description: Option<String>,
}
impl FoundationResponse {
    pub fn from_entity(foundation: &entity::foundations::Model) -> Self {
        Self {
            id: foundation.id as i64,
            name: foundation.name.clone(),
            description: foundation.description.clone(),
            created_at: foundation.created_at.to_string(),
            //  Dipakai jika updated_at adalah Option (tipe: Option<DateTime>)
            // foundation.updated_at.as_ref().map(|dt| dt.to_string()),
            // Dipakai jika updated_at bukan Option (tipe: DateTime)
            updated_at: foundation.updated_at.to_string(),
        }
    }
}
