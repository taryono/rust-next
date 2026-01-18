use crate::{controllers::foundation_controller, middleware::auth::JwtMiddleware};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/foundations")
            .wrap(JwtMiddleware)
            .route("", web::get().to(foundation_controller::get_foundations))
            .route("/me", web::get().to(foundation_controller::get_current_foundation))
            .route("/me", web::put().to(foundation_controller::update_current_foundation))
            .route(
                "/deleted",
                web::get().to(foundation_controller::get_deleted_foundations),
            ) // ← Baru
            .route("/{id}", web::get().to(foundation_controller::get_foundation_by_id))
            .route("/{id}", web::delete().to(foundation_controller::delete_foundation)) // Soft delete
            .route(
                "/{id}/restore",
                web::post().to(foundation_controller::restore_foundation),
            ) // ← Baru
            .route(
                "/{id}/force",
                web::delete().to(foundation_controller::force_delete_foundation),
            ), // ← Baru
    );
}
