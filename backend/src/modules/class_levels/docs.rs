// ============================================================================
// docs.rs - OpenAPI Documentation
// ============================================================================
use super::dto::{CreateClassLevelRequest, ClassLevelResponse, UpdateClassLevelRequest};
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
            ClassLevelResponse,
            CreateClassLevelRequest,
            UpdateClassLevelRequest,
            PaginatedResponse<ClassLevelResponse>,
            PaginationParams,
        )
    ),
    tags(
        (name = "Class Level", description = "Class Level management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ClassLevelsApiDoc;

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
