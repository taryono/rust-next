use crate::{middleware::auth::JwtMiddleware, modules::roles::role};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/roles")
            .wrap(JwtMiddleware)
            .route("", web::get().to(role::get_roles))
            .route("/me", web::get().to(role::get_current_role))
            .route("/me", web::put().to(role::update_current_role))
            .route("/deleted", web::get().to(role::get_deleted_roles)) // ← Baru
            .route("/{id}", web::get().to(role::get_role_by_id))
            .route("/{id}", web::delete().to(role::delete_role)) // Soft delete
            .route("/{id}/restore", web::post().to(role::restore_role)) // ← Baru
            .route("/{id}/force", web::delete().to(role::force_delete_role)), // ← Baru
    );
}
