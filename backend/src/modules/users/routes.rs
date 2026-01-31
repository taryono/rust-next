// backend/src/modules/users/routes.rs
use crate::{middleware::auth::JwtMiddleware, modules::users::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware)
            .route("", web::get().to(handler::get_users))
            .route("/create", web::post().to(handler::create))
            .route("/me", web::get().to(handler::get_current_user))
            .route("/me", web::put().to(handler::update_user))
            .route("/change-password", web::post().to(handler::change_password))
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::delete().to(handler::delete_user)),
    );
}
