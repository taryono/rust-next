// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::academic::units::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/units")
            .wrap(JwtMiddleware)
            .route("/options", web::get().to(handler::get_options)) // ← harus duluan
            .route("", web::post().to(handler::create))
            .route("", web::get().to(handler::get_all))
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::put().to(handler::update))
            .route("/{id}", web::delete().to(handler::delete)),
    );
}
