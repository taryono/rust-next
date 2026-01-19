// src/docs/foundation_docs.rs
use crate::modules::foundations::{foundation, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        foundation::get_foundations,
        foundation::get_foundation_by_id,
        foundation::get_current_foundation,
        foundation::update_current_foundation,
        foundation::delete_foundation,
        foundation::restore_foundation,
        foundation::force_delete_foundation,
        foundation::get_deleted_foundations,
    ),
    components(
        schemas(
            models::FoundationResponse,
            models::FoundationListResponse,
            models::UpdateFoundationRequest,
        )
    ),
    tags(
        (name = "foundations", description = "Foundation management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct FoundationsApiDoc;

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
