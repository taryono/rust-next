use crate::modules::{
    academic_years,
    applicants,
    attendances,
    auth,
    class_levels,
    classes,
    departments,
    employees,
    foundations,
    permissions,
    positions,
    roles,
    rooms,
    semesters,
    settings,
    students,
    subjects,
    teachers,
    unit_types,
    units,
    user_profiles,
    users,
    // character_traits,
    // report_cards,
    // schedule_generation_logs,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    academic_years::routes::configure(cfg);
    applicants::routes::configure(cfg);
    attendances::routes::configure(cfg);
    auth::routes::configure(cfg);
    class_levels::routes::configure(cfg);
    classes::routes::configure(cfg);
    departments::routes::configure(cfg);
    employees::routes::configure(cfg);
    foundations::routes::configure(cfg);
    permissions::routes::configure(cfg);
    positions::routes::configure(cfg);
    roles::routes::configure(cfg);
    rooms::routes::configure(cfg);
    semesters::routes::configure(cfg);
    settings::routes::configure(cfg);
    students::routes::configure(cfg);
    subjects::routes::configure(cfg);
    teachers::routes::configure(cfg);
    unit_types::routes::configure(cfg);
    units::routes::configure(cfg);
    user_profiles::routes::configure(cfg); // ✅ duplikat dihapus
    users::routes::configure(cfg);
    // character_traits::routes::configure(cfg);
    // report_cards::routes::configure(cfg);
    // schedule_generation_logs::routes::configure(cfg);
}
