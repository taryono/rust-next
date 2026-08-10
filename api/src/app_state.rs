// api/src/app_state.rs
use crate::config::AppConfig;
// academic modules
use crate::modules::academic::academic_years::AcademicYearService;
use crate::modules::academic::attendances::AttendanceService;
use crate::modules::academic::class_levels::ClassLevelService;
use crate::modules::academic::classes::ClassService;
use crate::modules::academic::departments::DepartmentService;
use crate::modules::academic::employees::EmployeeService;
use crate::modules::academic::positions::PositionService;
use crate::modules::academic::regulations::RegulationService;
use crate::modules::academic::rooms::RoomService;
use crate::modules::academic::semesters::SemesterService;
use crate::modules::academic::subjects::SubjectService;
use crate::modules::academic::teachers::TeacherService;
use crate::modules::academic::unit_types::UnitTypeService;
use crate::modules::academic::units::UnitService;
// admission modules
use crate::modules::admission::applicants::ApplicantService;
use crate::modules::admission::registrations::RegistrationService;
// finance modules
use crate::modules::finance::invoices::InvoiceService;
use crate::modules::finance::payments::PaymentService;
// identity modules
use crate::modules::identity::auth::AuthService;
use crate::modules::identity::menus::MenuService;
use crate::modules::identity::permissions::PermissionService;
use crate::modules::identity::roles::RoleService;
use crate::modules::identity::user_profiles::UserProfileService;
use crate::modules::identity::users::UserService;
// institution
use crate::modules::institution::boarding_schools::BoardingSchoolService;
use crate::modules::institution::courses::CourseService;
use crate::modules::institution::schools::SchoolService;
use crate::modules::institution::sports::SportService;
use crate::modules::institution::universities::UniversityService;

// library modules
use crate::modules::library::authors::AuthorService;
use crate::modules::library::book_categories::BookCategoryService;
use crate::modules::library::book_copies::BookCopyService;
use crate::modules::library::books::BookService;
use crate::modules::library::publishers::PublisherService;
use crate::modules::library::borrowings::BorrowingService;

// platform modules
use crate::modules::platform::foundation_regulations::FoundationRegulationService;
use crate::modules::platform::foundation_types::FoundationTypeService;
use crate::modules::platform::foundations::FoundationService;
use crate::modules::platform::settings::SettingService;
use crate::modules::student::guardians::GuardianService;
use crate::modules::student::students::StudentService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,

    // Academic
    pub academic_year_service: Arc<AcademicYearService>,
    pub attendance_service: Arc<AttendanceService>,
    pub class_level_service: Arc<ClassLevelService>,
    pub class_service: Arc<ClassService>,
    pub department_service: Arc<DepartmentService>,
    pub employee_service: Arc<EmployeeService>,
    pub position_service: Arc<PositionService>,
    pub regulation_service: Arc<RegulationService>,
    pub room_service: Arc<RoomService>,
    pub semester_service: Arc<SemesterService>,
    pub subject_service: Arc<SubjectService>,
    pub teacher_service: Arc<TeacherService>,
    pub unit_service: Arc<UnitService>,
    pub unit_type_service: Arc<UnitTypeService>,

    // Admission
    pub applicant_service: Arc<ApplicantService>,
    pub registration_service: Arc<RegistrationService>,

    // Finance
    pub invoice_service: Arc<InvoiceService>,
    pub payment_service: Arc<PaymentService>,

    // Identity
    pub auth_service: Arc<AuthService>,
    pub menu_service: Arc<MenuService>,
    pub permission_service: Arc<PermissionService>,
    pub role_service: Arc<RoleService>,
    pub user_profile_service: Arc<UserProfileService>,
    pub user_service: Arc<UserService>,

    // Institution
    pub boarding_school_service: Arc<BoardingSchoolService>,
    pub course_service: Arc<CourseService>,
    pub school_service: Arc<SchoolService>,
    pub sport_service: Arc<SportService>,
    pub university_service: Arc<UniversityService>,

    // Library
    pub author_service: Arc<AuthorService>,
    pub book_copy_service: Arc<BookCopyService>,
    pub book_category_service: Arc<BookCategoryService>,
    pub book_service: Arc<BookService>,
    pub borrowing_service: Arc<BorrowingService>,
    pub publisher_service: Arc<PublisherService>,

    // Platform
    pub foundation_service: Arc<FoundationService>,
    pub foundation_regulation_service: Arc<FoundationRegulationService>,
    pub foundation_type_service: Arc<FoundationTypeService>,
    pub setting_service: Arc<SettingService>,

    // Student
    pub guardian_service: Arc<GuardianService>,
    pub student_service: Arc<StudentService>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        // Academic
        academic_year_service: AcademicYearService,
        attendance_service: AttendanceService,
        class_level_service: ClassLevelService,
        class_service: ClassService,
        department_service: DepartmentService,
        employee_service: EmployeeService,
        position_service: PositionService,
        regulation_service: RegulationService,
        room_service: RoomService,
        semester_service: SemesterService,
        subject_service: SubjectService,
        teacher_service: TeacherService,
        unit_service: UnitService,
        unit_type_service: UnitTypeService,
        // Admission
        applicant_service: ApplicantService,
        registration_service: RegistrationService,
        // Finance
        invoice_service: InvoiceService,
        payment_service: PaymentService,
        // Identity
        auth_service: AuthService,
        menu_service: MenuService,
        permission_service: PermissionService,
        role_service: RoleService,
        user_profile_service: UserProfileService,
        user_service: UserService,
        // Institution
        boarding_school_service: BoardingSchoolService,
        course_service: CourseService,
        school_service: SchoolService,
        sport_service: SportService,
        university_service: UniversityService,
        // Library
        author_service: AuthorService,
        book_copy_service: BookCopyService,
        book_category_service: BookCategoryService,
        book_service: BookService,
        borrowing_service: BorrowingService,
        publisher_service: PublisherService,
        // Platform
        foundation_service: FoundationService,
        foundation_regulation_service: FoundationRegulationService,
        foundation_type_service: FoundationTypeService,
        setting_service: SettingService,
        // Student
        guardian_service: GuardianService,
        student_service: StudentService,
    ) -> Self {
        Self {
            config,
            // Academic
            academic_year_service: Arc::new(academic_year_service),
            attendance_service: Arc::new(attendance_service),
            class_level_service: Arc::new(class_level_service),
            class_service: Arc::new(class_service),
            department_service: Arc::new(department_service),
            employee_service: Arc::new(employee_service),
            position_service: Arc::new(position_service),
            regulation_service: Arc::new(regulation_service),
            room_service: Arc::new(room_service),
            semester_service: Arc::new(semester_service),
            subject_service: Arc::new(subject_service),
            teacher_service: Arc::new(teacher_service),
            unit_service: Arc::new(unit_service),
            unit_type_service: Arc::new(unit_type_service),
            // Admission
            applicant_service: Arc::new(applicant_service),
            registration_service: Arc::new(registration_service),
            // Finance
            invoice_service: Arc::new(invoice_service),
            payment_service: Arc::new(payment_service),
            // Identity
            auth_service: Arc::new(auth_service),
            menu_service: Arc::new(menu_service),
            permission_service: Arc::new(permission_service),
            role_service: Arc::new(role_service),
            user_profile_service: Arc::new(user_profile_service),
            user_service: Arc::new(user_service),
            // Institution
            boarding_school_service: Arc::new(boarding_school_service),
            course_service: Arc::new(course_service),
            school_service: Arc::new(school_service),
            sport_service: Arc::new(sport_service),
            university_service: Arc::new(university_service),
            // Library
            author_service: Arc::new(author_service),
            book_copy_service: Arc::new(book_copy_service),
            book_category_service: Arc::new(book_category_service),
            book_service: Arc::new(book_service),
            borrowing_service: Arc::new(borrowing_service),
            publisher_service: Arc::new(publisher_service),

            // Platform
            foundation_service: Arc::new(foundation_service),
            foundation_regulation_service: Arc::new(foundation_regulation_service),
            foundation_type_service: Arc::new(foundation_type_service),
            setting_service: Arc::new(setting_service),
            // Student
            guardian_service: Arc::new(guardian_service),
            student_service: Arc::new(student_service),
        }
    }
}
