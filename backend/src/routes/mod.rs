use actix_web::web;
pub fn init(cfg: &mut web::ServiceConfig) {
    crate::modules::auth::routes::configure(cfg);
    crate::modules::users::routes::configure(cfg);
    crate::modules::roles::routes::configure(cfg);
    crate::modules::foundations::routes::configure(cfg);
    crate::modules::academic_years::routes::configure(cfg);
    crate::modules::permissions::routes::configure(cfg);
    crate::modules::teachers::routes::configure(cfg);
    crate::modules::subjects::routes::configure(cfg);
    crate::modules::units::routes::configure(cfg);
    crate::modules::students::routes::configure(cfg);
    crate::modules::class_levels::routes::configure(cfg);
    crate::modules::classes::routes::configure(cfg);
    crate::modules::attendances::routes::configure(cfg);
    crate::modules::applicants::routes::configure(cfg);
    crate::modules::departments::routes::configure(cfg);
    crate::modules::user_profiles::routes::configure(cfg);
    crate::modules::semesters::routes::configure(cfg);
    crate::modules::settings::routes::configure(cfg);
    // crate::modules::character_traits::routes::configure(cfg);
    // crate::modules::report_cards::routes::configure(cfg);
    // crate::modules::schedule_generation_logs::routes::configure(cfg);
    crate::modules::rooms::routes::configure(cfg);
    crate::modules::employees::routes::configure(cfg);
    crate::modules::positions::routes::configure(cfg);
    crate::modules::unit_types::routes::configure(cfg);
    crate::modules::user_profiles::routes::configure(cfg);
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    init(cfg);
}
