// src/app_state.rs
use crate::modules::academic_years::AcademicYearService;
use crate::modules::applicants::ApplicantService;
use crate::modules::attendances::AttendanceService;
use crate::modules::auth::AuthService;
use crate::modules::class_levels::ClassLevelService;
use crate::modules::classes::ClassService;
use crate::modules::departments::DepartmentService;
use crate::modules::employees::EmployeeService;
use crate::modules::foundations::FoundationService;
use crate::modules::permissions::PermissionService;
use crate::modules::positions::PositionService;
use crate::modules::roles::RoleService;
use crate::modules::rooms::RoomService;
use crate::modules::semesters::SemesterService;
use crate::modules::students::StudentService;
use crate::modules::subjects::SubjectService;
use crate::modules::teachers::TeacherService;
use crate::modules::unit_types::UnitTypeService;
use crate::modules::units::UnitService;
use crate::modules::users::UserService;
use std::sync::Arc;
pub struct AppState {
    pub academic_year_service: Arc<AcademicYearService>,
    pub applicant_service: Arc<ApplicantService>,
    pub attendance_service: Arc<AttendanceService>,
    pub auth_service: Arc<AuthService>, // ← Tambahkan ini
    pub class_level_service: Arc<ClassLevelService>,
    pub class_service: Arc<ClassService>,
    pub department_service: Arc<DepartmentService>,
    pub employee_service: Arc<EmployeeService>,
    pub foundation_service: Arc<FoundationService>,
    pub permission_service: Arc<PermissionService>,
    pub position_service: Arc<PositionService>,
    pub role_service: Arc<RoleService>,
    pub room_service: Arc<RoomService>,
    pub semester_service: Arc<SemesterService>,
    pub student_service: Arc<StudentService>,
    pub subject_service: Arc<SubjectService>,
    pub teacher_service: Arc<TeacherService>,
    pub unit_service: Arc<UnitService>,
    pub unit_type_service: Arc<UnitTypeService>,
    pub user_service: Arc<UserService>,
}
impl AppState {
    pub fn new(
        academic_year_service: AcademicYearService,
        applicant_service: ApplicantService,
        attendance_service: AttendanceService,
        auth_service: AuthService,
        class_level_service: ClassLevelService,
        class_service: ClassService,
        department_service: DepartmentService,
        employee_service: EmployeeService,
        foundation_service: FoundationService,
        permission_service: PermissionService,
        position_service: PositionService,
        role_service: RoleService,
        room_service: RoomService,
        semester_service: SemesterService,
        student_service: StudentService,
        subject_service: SubjectService,
        teacher_service: TeacherService,
        unit_service: UnitService,
        unit_type_service: UnitTypeService,
        user_service: UserService,
    ) -> Self {
        Self {
            academic_year_service: Arc::new(academic_year_service),
            applicant_service: Arc::new(applicant_service),
            attendance_service: Arc::new(attendance_service),
            auth_service: Arc::new(auth_service),
            class_level_service: Arc::new(class_level_service),
            class_service: Arc::new(class_service),
            employee_service: Arc::new(employee_service),
            department_service: Arc::new(department_service),
            foundation_service: Arc::new(foundation_service),
            permission_service: Arc::new(permission_service),
            position_service: Arc::new(position_service),
            role_service: Arc::new(role_service),
            room_service: Arc::new(room_service),
            semester_service: Arc::new(semester_service),
            student_service: Arc::new(student_service),
            subject_service: Arc::new(subject_service),
            teacher_service: Arc::new(teacher_service),
            unit_service: Arc::new(unit_service),
            unit_type_service: Arc::new(unit_type_service),
            user_service: Arc::new(user_service),
        }
    }
}
