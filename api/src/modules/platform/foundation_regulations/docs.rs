// ============================================================================
// docs.rs - OpenAPI Documentation
// api/src/modules/foundation_regulations/docs.rs
// ============================================================================
use super::dto::{
    CreateFoundationRegulationRequest, FoundationRegulationResponse,
    ToggleFoundationRegulationRequest, UpdateFoundationRegulationConfigRequest,
};
use super::handler;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Admin
        handler::get_all,
        handler::get_by_id,
        // Per Yayasan
        handler::get_by_foundation_code,
        handler::create,
        handler::toggle,
        handler::update_config,
        handler::delete,
    ),
    components(
        schemas(
            FoundationRegulationResponse,
            CreateFoundationRegulationRequest,
            ToggleFoundationRegulationRequest,
            UpdateFoundationRegulationConfigRequest,
            PaginatedResponse<FoundationRegulationResponse>,
            PaginationParams,
        )
    ),
    tags(
        (name = "Foundation Regulations", description = "Manajemen regulasi per yayasan")
    ),
    modifiers(&SecurityAddon)
)]
pub struct FoundationRegulationsApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}
