// ============================================
// 4. src/docs/academic_year_docs.rs
// ============================================
use crate::modules::academic_years::academic_year;
use crate::modules::academic_years::models::AcademicYearResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        academic_year::get_all,
        academic_year::get_by_id,
        academic_year::create,
        academic_year::update,
        academic_year::delete,
    ),
    components(
        schemas(
            AcademicYearResponse,
        )
    ),
    tags(
        (name = "academic_years", description = "Academic Year management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct AcademicYearsApiDoc;

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
