// api/src/app_state.rs
use crate::config::AppConfig;
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
use crate::modules::admission::applicants::ApplicantService;
use crate::modules::admission::registrations::RegistrationService;
use crate::modules::finance::invoices::InvoiceService;
use crate::modules::finance::payments::PaymentService;
use crate::modules::identity::auth::AuthService;
use crate::modules::identity::menus::MenuService;
use crate::modules::identity::permissions::PermissionService;
use crate::modules::identity::roles::RoleService;
use crate::modules::identity::user_profiles::UserProfileService;
use crate::modules::identity::users::UserService;
use crate::modules::institution::boarding_schools::BoardingSchoolService;
use crate::modules::institution::courses::CourseService;
use crate::modules::institution::schools::SchoolService;
use crate::modules::institution::sports::SportService;
use crate::modules::institution::universities::UniversityService;
use crate::modules::library::books::BookService;
use crate::modules::library::borrowongs::BorrowingService;
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
    pub academic_year_service: Arc<AcademicYearService>,
    pub boarding_school_service: Arc<BoardingSchoolService>,
    pub book_service: Arc<BookService>,
    pub borrowong_service: Arc<BorrowingService>,
    pub course_service: Arc<CourseService>,
    pub school_service: Arc<SchoolService>,
    pub sport_service: Arc<SportService>,
    pub university_service: Arc<UniversityService>,
    pub guardian_service: Arc<GuardianService>,
    pub applicant_service: Arc<ApplicantService>,
    pub registration_service: Arc<RegistrationService>,
    pub invoice_service: Arc<InvoiceService>,
    pub payment_service: Arc<PaymentService>,
    pub attendance_service: Arc<AttendanceService>,
    pub auth_service: Arc<AuthService>,
    pub class_level_service: Arc<ClassLevelService>,
    pub class_service: Arc<ClassService>,
    pub department_service: Arc<DepartmentService>,
    pub employee_service: Arc<EmployeeService>,
    pub foundation_service: Arc<FoundationService>,
    pub foundation_regulation_service: Arc<FoundationRegulationService>,
    pub foundation_type_service: Arc<FoundationTypeService>,
    pub menu_service: Arc<MenuService>,
    pub permission_service: Arc<PermissionService>,
    pub position_service: Arc<PositionService>,
    pub regulation_service: Arc<RegulationService>,
    pub role_service: Arc<RoleService>,
    pub room_service: Arc<RoomService>,
    pub semester_service: Arc<SemesterService>,
    pub setting_service: Arc<SettingService>,
    pub student_service: Arc<StudentService>,
    pub subject_service: Arc<SubjectService>,
    pub teacher_service: Arc<TeacherService>,
    pub unit_service: Arc<UnitService>,
    pub unit_type_service: Arc<UnitTypeService>,
    pub user_profile_service: Arc<UserProfileService>,
    pub user_service: Arc<UserService>,
}
impl AppState {
    pub fn new(
        config: AppConfig,
        academic_year_service: AcademicYearService,
        applicant_service: ApplicantService,
        attendance_service: AttendanceService,
        auth_service: AuthService,
        class_level_service: ClassLevelService,
        class_service: ClassService,
        department_service: DepartmentService,
        employee_service: EmployeeService,
        foundation_service: FoundationService,
        foundation_regulation_service: FoundationRegulationService,
        foundation_type_service: FoundationTypeService,
        menu_service: MenuService,
        permission_service: PermissionService,
        position_service: PositionService,
        regulation_service: RegulationService,
        role_service: RoleService,
        room_service: RoomService,
        semester_service: SemesterService,
        setting_service: SettingService,
        student_service: StudentService,
        subject_service: SubjectService,
        teacher_service: TeacherService,
        unit_service: UnitService,
        unit_type_service: UnitTypeService,
        user_profile_service: UserProfileService,
        user_service: UserService,
    ) -> Self {
        Self {
            config,
            academic_year_service: Arc::new(academic_year_service),
            applicant_service: Arc::new(applicant_service),
            attendance_service: Arc::new(attendance_service),
            auth_service: Arc::new(auth_service),
            class_level_service: Arc::new(class_level_service),
            class_service: Arc::new(class_service),
            department_service: Arc::new(department_service),
            employee_service: Arc::new(employee_service),
            foundation_service: Arc::new(foundation_service),
            borrowong_service: Arc::new(BorrowingService::default()),
            boarding_school_service: Arc::new(BoardingSchoolService::default()),
            book_service: Arc::new(BookService::default()),
            course_service: Arc::new(CourseService::default()),
            school_service: Arc::new(SchoolService::default()),
            sport_service: Arc::new(SportService::default()),
            university_service: Arc::new(UniversityService::default()),
            guardian_service: Arc::new(GuardianService::default()),
            invoice_service: Arc::new(InvoiceService::default()),
            payment_service: Arc::new(PaymentService::default()),
            registration_service: Arc::new(RegistrationService::default()),
            foundation_regulation_service: Arc::new(foundation_regulation_service),
            foundation_type_service: Arc::new(foundation_type_service),
            menu_service: Arc::new(menu_service),
            permission_service: Arc::new(permission_service),
            position_service: Arc::new(position_service),
            regulation_service: Arc::new(regulation_service),
            role_service: Arc::new(role_service),
            room_service: Arc::new(room_service),
            semester_service: Arc::new(semester_service),
            setting_service: Arc::new(setting_service),
            student_service: Arc::new(student_service),
            subject_service: Arc::new(subject_service),
            teacher_service: Arc::new(teacher_service),
            unit_service: Arc::new(unit_service),
            unit_type_service: Arc::new(unit_type_service),
            user_profile_service: Arc::new(user_profile_service),
            user_service: Arc::new(user_service),
        }
    }
}
