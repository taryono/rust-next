// src/modules/auth/routes.rs
use crate::modules::auth::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/register", web::post().to(handler::register))
            .route("/login", web::post().to(handler::login))
            .route("/refresh", web::post().to(handler::refresh)),
    );
}
