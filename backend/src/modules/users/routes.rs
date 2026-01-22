use crate::{middleware::auth::JwtMiddleware, modules::users::user};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware)
            .route("", web::get().to(user::get_users))
            .route("/me", web::get().to(user::get_current_user))
            .route("/me", web::put().to(user::update_current_user))
            .route("/change-password", web::post().to(user::change_password))
            .route("/{id}", web::get().to(user::get_by_id))
            .route("/{id}", web::delete().to(user::delete_user)),
    );
}
