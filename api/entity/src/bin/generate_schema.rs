use entity::*;
use sea_orm::{DbBackend, Schema};

fn main() {
    let schema = Schema::new(DbBackend::MySql);
    let backend = DbBackend::MySql;

    println!("-- Auto-generated schema from SeaORM entities");
    println!("SET FOREIGN_KEY_CHECKS = 0;");

    {
        let mut stmt = schema.create_table_from_entity(academic_calendars::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(academic_years::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(applicants::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(attendances::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(authors::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(boarding_schools::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(book_categories::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(book_copies::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(books::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(borrowings::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(character_assessments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(character_traits::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(class_levels::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(class_schedules::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(class_subjects::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(classes::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(classrooms::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(courses::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(departments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(employees::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(extracurricular_activities::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(extracurricular_enrollments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(foundation_regulations::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(foundation_types::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(foundations::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(grades::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(guardians::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(holidays::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(homeroom_teachers::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(homerooms::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(invoices::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(levels::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(members::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(menu_permissions::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(menu_roles::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(menus::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(notifications::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(payments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(permissions::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(positions::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(publishers::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(registrations::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(regulations::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(report_cards::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(role_permissions::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(role_users::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(roles::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(room_facilities::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(room_types::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(rooms::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(salary_grades::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(salary_payments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(schedule_generation_logs::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(schools::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(scopes::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(semesters::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(settings::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(sports::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(student_enrollments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(students::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(subject_preferences::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(subject_room_requirements::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(subjects::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teacher_assignments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teacher_availability::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teacher_salaries::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teacher_subjects::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teachers::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(teachers_payments::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(time_slots::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(unit_types::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(units::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(universities::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(user_permissions::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(user_profiles::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }
    {
        let mut stmt = schema.create_table_from_entity(users::Entity);
        stmt.if_not_exists();
        println!("{};", backend.build(&stmt).to_string());
    }

    println!("SET FOREIGN_KEY_CHECKS = 1;");
}
