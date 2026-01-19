use crate::{middleware::auth::JwtMiddleware, modules::foundations::foundation};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/foundations")
            .wrap(JwtMiddleware)
            .route("", web::get().to(foundation::get_foundations))
            .route("/me", web::get().to(foundation::get_current_foundation))
            .route("/me", web::put().to(foundation::update_current_foundation))
            .route(
                "/deleted",
                web::get().to(foundation::get_deleted_foundations),
            ) // ← Baru
            .route("/{id}", web::get().to(foundation::get_foundation_by_id))
            .route("/{id}", web::delete().to(foundation::delete_foundation)) // Soft delete
            .route(
                "/{id}/restore",
                web::post().to(foundation::restore_foundation),
            ) // ← Baru
            .route(
                "/{id}/force",
                web::delete().to(foundation::force_delete_foundation),
            ), // ← Baru
    );
}
