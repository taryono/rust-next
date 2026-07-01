// ============================================================================
// routes.rs - Route Configuration
// ============================================================================
// use crate::{middleware::auth::JwtMiddleware, modules::menus::handler};
// use actix_web::web;

// pub fn configure(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/api/menus")
//             .wrap(JwtMiddleware)
//             .route("", web::post().to(handler::create))
//             .route("", web::get().to(handler::get_all))
//             .route("/{id}", web::get().to(handler::get_by_id))
//             .route("/{id}", web::put().to(handler::update))
//             .route("/{id}", web::delete().to(handler::delete))
//             .route("/my-menus", web::get().to(handler::get_my_menus))
//     );
// }

use crate::{middleware::auth::JwtMiddleware, modules::menus::handler};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/menus")
            .wrap(JwtMiddleware)
            .route("", web::post().to(handler::create))
            .route("", web::get().to(handler::get_all))
            .route("/my-menus", web::get().to(handler::get_my_menus)) // ← fix: delete → get, path juga diperbaiki
            .route("/{id}", web::get().to(handler::get_by_id))
            .route("/{id}", web::put().to(handler::update))
            .route("/{id}", web::delete().to(handler::delete)),
    );
}
