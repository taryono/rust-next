use crate::{controllers::role_controller, middleware::auth::JwtMiddleware};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/roles")
            .wrap(JwtMiddleware)
            .route("", web::get().to(role_controller::get_roles))
            .route("/me", web::get().to(role_controller::get_current_role))
            .route("/me", web::put().to(role_controller::update_current_role))
            .route(
                "/deleted",
                web::get().to(role_controller::get_deleted_roles),
            ) // ← Baru
            .route("/{id}", web::get().to(role_controller::get_role_by_id))
            .route("/{id}", web::delete().to(role_controller::delete_role)) // Soft delete
            .route(
                "/{id}/restore",
                web::post().to(role_controller::restore_role),
            ) // ← Baru
            .route(
                "/{id}/force",
                web::delete().to(role_controller::force_delete_role),
            ), // ← Baru
    );
}
