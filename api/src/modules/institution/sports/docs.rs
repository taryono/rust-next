// api/src/modules/institution/sport/docs.rs

// ============================================================================
// docs.rs - OpenAPI Documentation
// ============================================================================
use super::dto::{CreateSportRequest, SportResponse, UpdateSportRequest};
use super::handler;
use crate::utils::pagination::{PaginatedResponse, PaginationParams};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::get_all,
        handler::get_by_id, 
        handler::create,
        handler::update,
        handler::delete,
    ),
    components(
        schemas(
            SportResponse,
            CreateSportRequest,
            UpdateSportRequest,
            PaginatedResponse<SportResponse>,
            PaginationParams,
        )
    ),
    tags(
        (name = "Sport ", description = "Sport management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct SportsApiDoc;

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
