// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::teachers::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/teachers")
            .wrap(JwtMiddleware)
            .route("", web::post().to(handler::create))
            .route("", web::get().to(handler::get_all))
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::put().to(handler::update))
            .route("/{id}", web::delete().to(handler::delete)),
    );
}
