// ============================================================================
// routes.rs - Route Configuration
// api/src/modules/foundation_regulations/routes.rs
// ============================================================================
use crate::{middleware::auth::JwtMiddleware, modules::platform::foundation_regulations::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // ----------------------------------------------------------------
    // Admin — akses langsung ke foundation_regulations
    // GET  /api/foundation-regulations
    // GET  /api/foundation-regulations/{id}
    // ----------------------------------------------------------------
    cfg.service(
        web::scope("/api/foundation-regulations")
            .wrap(JwtMiddleware)
            .route("", web::get().to(handler::get_all))
            .route("/{id}", web::get().to(handler::get_by_id)),
    );

    // ----------------------------------------------------------------
    // Per Yayasan
    // GET    /api/foundations/{foundation_code}/regulations
    // POST   /api/foundations/{foundation_code}/regulations/{regulation_code}
    // DELETE /api/foundations/{foundation_code}/regulations/{regulation_code}
    // PUT    /api/foundations/{foundation_code}/regulations/{regulation_code}/toggle
    // PUT    /api/foundations/{foundation_code}/regulations/{regulation_code}/config
    // ----------------------------------------------------------------
    cfg.service(
        web::scope("/api/foundations/{foundation_code}/regulations")
            .wrap(JwtMiddleware)
            .route("", web::get().to(handler::get_by_foundation_code))
            .route(
                "/{regulation_code}",
                web::post().to(handler::create),
            )
            .route(
                "/{regulation_code}",
                web::delete().to(handler::delete),
            )
            .route(
                "/{regulation_code}/toggle",
                web::put().to(handler::toggle),
            )
            .route(
                "/{regulation_code}/config",
                web::put().to(handler::update_config),
            ),
    );
}
