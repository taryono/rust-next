// ====================== ACADEMIC MODULE ======================
use crate::modules::academic::{
    academic_years, attendances, class_levels, classes, departments, employees, positions,
    regulations, rooms, semesters, subjects, teachers, unit_types, units,
};
// ====================== ADMISSION MODULE ======================
use crate::modules::admission::{applicants, registrations};
// ====================== FINANCE MODULE ======================
use crate::modules::finance::{invoices, payments};
// ====================== IDENTITY MODULE ======================
use crate::modules::identity::{auth, menus, permissions, roles, user_profiles, users};
// ====================== INSTITUTION MODULE ======================
use crate::modules::institution::{boarding_schools, courses, schools, sports, universities};
// ====================== LIBRARY MODULE ======================
use crate::modules::library::{books, borrowings};
// ====================== PLATFORM MODULE ======================
use crate::modules::platform::{foundation_regulations, foundation_types, foundations, settings};
// ====================== STUDENT MODULE ======================
use crate::modules::student::{guardians, students};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // ACADEMIC
    academic_years::routes::configure(cfg);
    attendances::routes::configure(cfg);
    class_levels::routes::configure(cfg);
    classes::routes::configure(cfg);
    departments::routes::configure(cfg);
    employees::routes::configure(cfg);
    positions::routes::configure(cfg);
    regulations::routes::configure(cfg);
    rooms::routes::configure(cfg);
    semesters::routes::configure(cfg);
    subjects::routes::configure(cfg);
    teachers::routes::configure(cfg);
    unit_types::routes::configure(cfg);
    units::routes::configure(cfg);

    // ADMISSION
    applicants::routes::configure(cfg);
    registrations::routes::configure(cfg);

    // FINANCE
    invoices::routes::configure(cfg);
    payments::routes::configure(cfg);

    // IDENTITY
    auth::routes::configure(cfg);
    menus::routes::configure(cfg);
    permissions::routes::configure(cfg);
    roles::routes::configure(cfg);
    user_profiles::routes::configure(cfg); // ✅ duplikat dihapus
    users::routes::configure(cfg);

    // INSTITUTION
    boarding_schools::routes::configure(cfg);
    courses::routes::configure(cfg);
    schools::routes::configure(cfg);
    sports::routes::configure(cfg);
    universities::routes::configure(cfg);
    // LIBRARY
    books::routes::configure(cfg);
    borrowings::routes::configure(cfg);

    // PLATFORM
    foundations::routes::configure(cfg);
    foundation_regulations::routes::configure(cfg);
    foundation_types::routes::configure(cfg);
    settings::routes::configure(cfg);

    // STUDENT
    guardians::routes::configure(cfg);
    students::routes::configure(cfg);

    // character_traits::routes::configure(cfg);
    // report_cards::routes::configure(cfg);
    // schedule_generation_logs::routes::configure(cfg);
}
