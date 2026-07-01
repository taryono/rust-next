// api/src/modules/users/routes.rs
use crate::{middleware::auth::JwtMiddleware, modules::identity::users::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware)
            .route("", web::get().to(handler::get_users))
            .route("", web::post().to(handler::create))
            .route("/me", web::get().to(handler::get_current_user))
            .route("/me", web::put().to(handler::update))
            .route("/change-password", web::post().to(handler::change_password))
            .route("/multipart", web::post().to(handler::create_multipart))
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::delete().to(handler::delete_user))
            .route("/{id}/force", web::delete().to(handler::force_delete_user))
            .route("/{id}/restore", web::post().to(handler::restore_user))
            // Role management
            .route("/{id}/roles", web::get().to(handler::get_user_with_roles))
            .route("/{id}/roles", web::post().to(handler::assign_role))
            .route("/{id}/roles", web::put().to(handler::sync_roles))
            .route(
                "/{id}/roles/{role_id}",
                web::delete().to(handler::remove_role),
            ),
    );
}
