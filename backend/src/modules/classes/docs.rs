// ============================================================================
// docs.rs - OpenAPI Documentation
// ============================================================================
use super::dto::{CreateClassRequest, ClassResponse, UpdateClassRequest};
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
            ClassResponse,
            CreateClassRequest,
            UpdateClassRequest,
            PaginatedResponse<ClassResponse>,
            PaginationParams,
        )
    ),
    tags(
        (name = "Class ", description = "Class management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ClassesApiDoc;

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
