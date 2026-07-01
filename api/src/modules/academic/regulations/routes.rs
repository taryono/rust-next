// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::regulations::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/regulations")
            .wrap(JwtMiddleware)
            .route("", web::post().to(handler::create))
            .route("", web::get().to(handler::get_all))
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::put().to(handler::update))
            .route("/{id}", web::delete().to(handler::delete))
            // Foundation regulations
            .route(
                "/foundations/{foundation_code}/regulations",
                web::get().to(handler::get_foundation_regulations),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}/toggle",
                web::put().to(handler::toggle_foundation_regulation),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}/config",
                web::put().to(handler::update_foundation_regulation_config),
            ),
    );
}
