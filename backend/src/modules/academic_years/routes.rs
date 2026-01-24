// backend/src/modules/academic_years/routes.rs
// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::academic_years::handler};
use actix_web::web;
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/academic_years")
            .wrap(JwtMiddleware)
            .route("", web::post().to(handler::create))
            .route("", web::get().to(handler::get_all))
            .route(
                "/active/{foundation_id}",
                web::get().to(handler::get_active),
            )
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::put().to(handler::update))
            .route("/{id}", web::delete().to(handler::delete)),
    );
}
