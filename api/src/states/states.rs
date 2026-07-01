// src/states/state.rs
use crate::config::database::Database;
use crate::config::AppConfig;
use crate::AppState;
use actix_web::web;

pub fn init_app(db: Database) -> Result<web::Data<AppState>, Box<dyn std::error::Error>> {
    let config = AppConfig::from_env(); // ← inisialisasi config
    let applicant_service = crate::modules::admission::applicants::init_service(db.clone());
    let auth_service = crate::modules::identity::auth::init_service(db.clone());
    let academic_year_service = crate::modules::academic::academic_years::init_service(db.clone());
    let attendance_service = crate::modules::academic::attendances::init_service(db.clone());
    let boarding_school_service =
        crate::modules::institution::boarding_schools::init_service(db.clone());
    let course_service = crate::modules::institution::courses::init_service(db.clone());
    let school_service = crate::modules::institution::schools::init_service(db.clone());
    let sport_service = crate::modules::institution::sports::init_service(db.clone());
    let university_service = crate::modules::institution::universities::init_service(db.clone());
    let book_service = crate::modules::library::books::init_service(db.clone());
    let borrowing_service = crate::modules::library::borrowings::init_service(db.clone());
    let class_level_service = crate::modules::academic::class_levels::init_service(db.clone());
    let class_service = crate::modules::academic::classes::init_service(db.clone());
    let department_service = crate::modules::academic::departments::init_service(db.clone());
    let employee_service = crate::modules::academic::employees::init_service(db.clone());
    let guardian_service = crate::modules::student::guardians::init_service(db.clone());
    let foundation_service = crate::modules::platform::foundations::init_service(db.clone());
    let foundation_regulation_service =
        crate::modules::platform::foundation_regulations::init_service(db.clone());
    let foundation_type_service =
        crate::modules::platform::foundation_types::init_service(db.clone());
    let menu_service = crate::modules::identity::menus::init_service(db.clone());
    let permission_service = crate::modules::identity::permissions::init_service(db.clone());
    let position_service = crate::modules::academic::positions::init_service(db.clone());
    let regulation_service = crate::modules::academic::regulations::init_service(db.clone());
    let role_service = crate::modules::identity::roles::init_service(db.clone());
    let room_service = crate::modules::academic::rooms::init_service(db.clone());
    let semester_service = crate::modules::academic::semesters::init_service(db.clone());
    let setting_service = crate::modules::platform::settings::init_service(db.clone());
    let student_service = crate::modules::student::students::init_service(db.clone());
    let subject_service = crate::modules::academic::subjects::init_service(db.clone());
    let teacher_service = crate::modules::academic::teachers::init_service(db.clone());
    let unit_service = crate::modules::academic::units::init_service(db.clone());
    let unit_type_service = crate::modules::academic::unit_types::init_service(db.clone());
    let user_profile_service = crate::modules::identity::user_profiles::init_service(db.clone());
    let user_service = crate::modules::identity::users::init_service(db.clone());
    let guardian_service = crate::modules::student::guardians::init_service(db);
    // ✨ Create AppState

    Ok(web::Data::new(AppState::new(
        config,
        academic_year_service,
        applicant_service,
        attendance_service,
        auth_service,
        class_level_service,
        class_service,
        department_service,
        employee_service,
        foundation_service,
        foundation_regulation_service,
        foundation_type_service,
        menu_service,
        permission_service,
        position_service,
        regulation_service,
        role_service,
        room_service,
        semester_service,
        setting_service,
        student_service,
        subject_service,
        teacher_service,
        unit_service,
        unit_type_service,
        user_profile_service,
        user_service,
    )))
}
