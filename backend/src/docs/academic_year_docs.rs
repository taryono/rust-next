// ============================================
// 4. src/docs/academic_year_docs.rs
// ============================================
use crate::{controllers, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::academic_year_controller::get_all,
        controllers::academic_year_controller::get_by_id,
        controllers::academic_year_controller::create,
        controllers::academic_year_controller::update,
        controllers::academic_year_controller::delete,
    ),
    components(
        schemas(
            models::academic_year::AcademicYearResponse,
        )
    ),
    tags(
        (name = "academic_years", description = "Academic Year management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct AcademicYearApiDoc;

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
