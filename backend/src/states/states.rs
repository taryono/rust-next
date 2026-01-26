// src/app_state.rs
use crate::config::database::Database;
use crate::AppState;
use actix_web::web;
pub fn init_app(db: Database) -> Result<web::Data<AppState>, Box<dyn std::error::Error>> {
    let applicant_service = crate::modules::applicants::init_service(db.clone());
    let auth_service = crate::modules::auth::init_service(db.clone());
    let academic_year_service = crate::modules::academic_years::init_service(db.clone());
    let attendance_service = crate::modules::attendances::init_service(db.clone());
    let class_level_service = crate::modules::class_levels::init_service(db.clone());
    let class_service = crate::modules::classes::init_service(db.clone());
    let department_service = crate::modules::departments::init_service(db.clone());
    let employee_service = crate::modules::employees::init_service(db.clone());
    let foundation_service = crate::modules::foundations::init_service(db.clone());
    let permission_service = crate::modules::permissions::init_service(db.clone());
    let position_service = crate::modules::positions::init_service(db.clone());
    let role_service = crate::modules::roles::init_service(db.clone());
    let room_service = crate::modules::rooms::init_service(db.clone());
    let semester_service = crate::modules::semesters::init_service(db.clone());
    let student_service = crate::modules::students::init_service(db.clone());
    let subject_service = crate::modules::subjects::init_service(db.clone());
    let teacher_service = crate::modules::teachers::init_service(db.clone());
    let unit_service = crate::modules::units::init_service(db.clone());
    let unit_type_service = crate::modules::unit_types::init_service(db.clone());

    let user_service = crate::modules::users::init_service(db.clone());
    // ✨ Create AppState

    Ok(web::Data::new(AppState::new(
        academic_year_service,
        applicant_service,
        attendance_service,
        auth_service,
        class_level_service,
        class_service,
        department_service,
        employee_service,
        foundation_service,
        permission_service,
        position_service,
        role_service,
        room_service,
        semester_service,
        student_service,
        subject_service,
        teacher_service,
        unit_service,
        unit_type_service,
        user_service,
    )))
}
