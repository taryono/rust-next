// ============================================================================
// routes.rs - Route Configuration
// api/src/modules/foundation_regulations/routes.rs
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::platform::foundation_regulations::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(JwtMiddleware)
            // ----------------------------------------------------------------
            // Admin — akses langsung ke foundation_regulations
            // GET  /api/foundation-regulations
            // GET  /api/foundation-regulations/{id}
            // ----------------------------------------------------------------
            .route("/foundation-regulations", web::get().to(handler::get_all))
            .route(
                "/foundation-regulations/{id}",
                web::get().to(handler::get_by_id),
            )
            // ----------------------------------------------------------------
            // Per Yayasan
            // GET    /api/foundations/{foundation_code}/regulations
            // POST   /api/foundations/{foundation_code}/regulations/{regulation_code}
            // DELETE /api/foundations/{foundation_code}/regulations/{regulation_code}
            // PUT    /api/foundations/{foundation_code}/regulations/{regulation_code}/toggle
            // PUT    /api/foundations/{foundation_code}/regulations/{regulation_code}/config
            // ----------------------------------------------------------------
            .route(
                "/foundations/{foundation_code}/regulations",
                web::get().to(handler::get_by_foundation_code),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}",
                web::post().to(handler::create),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}",
                web::delete().to(handler::delete),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}/toggle",
                web::put().to(handler::toggle),
            )
            .route(
                "/foundations/{foundation_code}/regulations/{regulation_code}/config",
                web::put().to(handler::update_config),
            ),
    );
}
