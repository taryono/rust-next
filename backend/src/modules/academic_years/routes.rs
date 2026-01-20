// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
use super::handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/academic-years")
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
    