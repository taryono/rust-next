// src/app_state.rs
use crate::config::database::Database;
use crate::AppState;
use actix_web::web;
pub fn init_app(db: Database) -> Result<web::Data<AppState>, Box<dyn std::error::Error>> {
    let auth_service = crate::modules::auth::init_service(db.clone());
    let academic_year_service = crate::modules::academic_years::init_service(db.clone());
    let permission_service = crate::modules::permissions::init_service(db.clone());
    let position_service = crate::modules::positions::init_service(db.clone());
    let employee_service = crate::modules::employees::init_service(db.clone());
    // ✨ Create AppState

    Ok(web::Data::new(AppState::new(
        academic_year_service,
        auth_service,
        permission_service,
        position_service,
        employee_service,
    )))
}
