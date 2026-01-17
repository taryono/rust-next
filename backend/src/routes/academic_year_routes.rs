use crate::{controllers::academic_year_controller, middleware::auth::JwtMiddleware};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/academic-years")
            .wrap(JwtMiddleware)
            .route("", web::get().to(academic_year_controller::get_all))
            .route("", web::post().to(academic_year_controller::create))
            .route("/{id}", web::get().to(academic_year_controller::get_by_id))
            .route("/{id}", web::put().to(academic_year_controller::update))
            .route("/{id}", web::delete().to(academic_year_controller::delete)),
    );
}
