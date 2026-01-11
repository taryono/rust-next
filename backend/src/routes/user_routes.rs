use crate::{controllers::user_controller, middleware::auth::JwtMiddleware};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .wrap(JwtMiddleware)
            .route("", web::get().to(user_controller::get_users))
            .route("/me", web::get().to(user_controller::get_current_user))
            .route("/me", web::put().to(user_controller::update_current_user))
            .route("/change-password", web::post().to(user_controller::change_password))
            .route("/{id}", web::get().to(user_controller::get_user_by_id))
            .route("/{id}", web::delete().to(user_controller::delete_user)),
    );
}