use crate::{middleware::auth::JwtMiddleware, modules::academic_years::academic_year};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/academic-years")
            .wrap(JwtMiddleware)
            .route("", web::get().to(academic_year::get_all))
            .route("", web::post().to(academic_year::create))
            .route("/{id}", web::get().to(academic_year::get_by_id))
            .route("/{id}", web::put().to(academic_year::update))
            .route("/{id}", web::delete().to(academic_year::delete)),
    );
}
