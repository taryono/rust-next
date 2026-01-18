// src/docs/foundation_docs.rs
use crate::{controllers, models};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::foundation_controller::get_foundations,
        controllers::foundation_controller::get_foundation_by_id,
        controllers::foundation_controller::get_current_foundation,
        controllers::foundation_controller::update_current_foundation,
        controllers::foundation_controller::delete_foundation,
        controllers::foundation_controller::restore_foundation,
        controllers::foundation_controller::force_delete_foundation,
        controllers::foundation_controller::get_deleted_foundations,
    ),
    components(
        schemas(
            models::foundation::FoundationResponse,
            models::foundation::FoundationListResponse,
            models::foundation::UpdateFoundationRequest,
        )
    ),
    tags(
        (name = "foundations", description = "Foundation management endpoints")
    )
)]
pub struct FoundationApiDoc;
