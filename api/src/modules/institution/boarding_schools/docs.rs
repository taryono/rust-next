// ============================================================================
// docs.rs - OpenAPI Documentation
// ============================================================================
// api/src/modules/institution/boarding_school/docs.rs
use super::dto::{CreateBoardingSchoolRequest, BoardingSchoolResponse, UpdateBoardingSchoolRequest};
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
            BoardingSchoolResponse,
            CreateBoardingSchoolRequest,
            UpdateBoardingSchoolRequest,
            PaginatedResponse<BoardingSchoolResponse>,
            PaginationParams,
        )
    ),
    tags(
        (name = "Boarding Schoole ", description = "Boarding Schoole management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct BoardingSchoolsApiDoc;

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
