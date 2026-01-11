use crate::controllers::auth_controller;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(auth_controller::register))
            .route("/login", web::post().to(auth_controller::login))
            .route("/refresh", web::post().to(auth_controller::refresh)),
    );
}