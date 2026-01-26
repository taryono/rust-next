import axios from 'axios';
import { cookies } from './cookies';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

let isRefreshing = false;
let failedQueue = [];

const processQueue = (error, token = null) => {
    failedQueue.forEach(prom => {
        if (error) {
            prom.reject(error);
        } else {
            prom.resolve(token);
        }
    });

    failedQueue = [];
};

// Create axios instance
const axiosInstance = axios.create({
    baseURL: API_URL,
    headers: {
        'Content-Type': 'application/json',
    },
});

// Request interceptor - add token to headers
axiosInstance.interceptors.request.use(
    (config) => {
        const token = cookies.getAccessToken();
        if (token) {
            config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
    },
    (error) => {
        return Promise.reject(error);
    }
);

// Response interceptor - handle token refresh
axiosInstance.interceptors.response.use(
    (response) => response.data,
    async (error) => {
        const originalRequest = error.config;

        // If error is 401 and we haven't tried to refresh yet
        if (error.response?.status === 401 && !originalRequest._retry) {
            if (isRefreshing) {
                // If already refreshing, queue this request
                return new Promise((resolve, reject) => {
                    failedQueue.push({ resolve, reject });
                })
                    .then(token => {
                        originalRequest.headers.Authorization = `Bearer ${token}`;
                        return axiosInstance(originalRequest);
                    })
                    .catch(err => {
                        return Promise.reject(err);
                    });
            }

            originalRequest._retry = true;
            isRefreshing = true;

            const refreshToken = cookies.getRefreshToken();

            if (!refreshToken) {
                // No refresh token, redirect to login
                cookies.clearAll();
                if (typeof window !== 'undefined') {
                    window.location.href = '/login';
                }
                return Promise.reject(error);
            }

            try {
                // Call refresh endpoint
                const response = await axios.post(`${API_URL}/api/auth/refresh`, {
                    refresh_token: refreshToken
                });

                if (response.data.success && response.data.data) {
                    const { access_token, refresh_token } = response.data.data;

                    // Save new tokens to cookies
                    cookies.setTokens(access_token, refresh_token);

                    // Update authorization header
                    axiosInstance.defaults.headers.common['Authorization'] = `Bearer ${access_token}`;
                    originalRequest.headers.Authorization = `Bearer ${access_token}`;

                    // Process queued requests
                    processQueue(null, access_token);

                    // Retry original request
                    return axiosInstance(originalRequest);
                }
            } catch (refreshError) {
                processQueue(refreshError, null);

                // Refresh failed, clear tokens and redirect
                cookies.clearAll();
                if (typeof window !== 'undefined') {
                    window.location.href = '/login';
                }

                return Promise.reject(refreshError);
            } finally {
                isRefreshing = false;
            }
        }

        return Promise.reject(error);
    }
);

// API methods
export const api = {
    // Auth
    isOnline: () => axiosInstance.get('/health'),
    login: (data) => axiosInstance.post('/api/auth/login', data),
    register: (data) => axiosInstance.post('/api/auth/register', data),
    refresh: (refreshToken) => axiosInstance.post('/api/auth/refresh', { refresh_token: refreshToken }),

    // Users
    getUsers: (params = '') => axiosInstance.get(`/api/users${params}`),
    getUserById: (id) => axiosInstance.get(`/api/users/${id}`),
    getCurrentUser: () => axiosInstance.get('/api/users/me'), 
    updateCurrentUser: (data) => axiosInstance.put('/api/users/me', data),
    changePassword: (data) => axiosInstance.post('/api/users/change-password', data),
    deleteUser: (id) => axiosInstance.delete(`/api/users/${id}`),
 
    // Foundations 
    getFoundations: (params = '') => axiosInstance.get(`/api/foundations${params}`),
    getFoundationById: (id) => axiosInstance.get(`/api/foundation/${id}`),
    createFoundation: (data) => axiosInstance.post('/api/foundation', data),
    updateFoundation: (id, data) => axiosInstance.put(`/api/foundation/${id}`, data),
    deleteFoundation: (id) => axiosInstance.delete(`/api/foundation/${id}`),

    // Units 
    getUnits: (params = '') => axiosInstance.get(`/api/units${params}`),
    getUnitById: (id) => axiosInstance.get(`/api/unit/${id}`),
    createUnit: (data) => axiosInstance.post('/api/unit', data),
    updateUnit: (id, data) => axiosInstance.put(`/api/unit/${id}`, data),
    deleteUnit: (id) => axiosInstance.delete(`/api/unit/${id}`),

    // UnitTypes 
    getUnitTypes: () => axiosInstance.get('/api/unit-types'),
    getUnitTypeById: (id) => axiosInstance.get(`/api/unit-type/${id}`),
    createUnitType: (data) => axiosInstance.post('/api/unit-type', data),
    updateUnitType: (id, data) => axiosInstance.put(`/api/unit-type/${id}`, data),
    deleteUnitType: (id) => axiosInstance.delete(`/api/unit-type/${id}`),

     // Employees 
    getEmployees: (params = '') => axiosInstance.get(`/api/employees${params}`),
    getEmployeeById: (id) => axiosInstance.get(`/api/employee/${id}`),
    createEmployee: (data) => axiosInstance.post('/api/employee', data),
    updateEmployee: (id, data) => axiosInstance.put(`/api/employee/${id}`, data),
    deleteEmployee: (id) => axiosInstance.delete(`/api/employee/${id}`),

     // Teachers 
    getTeachers: () => axiosInstance.get('/api/teachers'),
    getTeacherById: (id) => axiosInstance.get(`/api/teacher/${id}`),
    createTeacher: (data) => axiosInstance.post('/api/teacher', data),
    updateTeacher: (id, data) => axiosInstance.put(`/api/teacher/${id}`, data),
    deleteTeacher: (id) => axiosInstance.delete(`/api/teacher/${id}`),

     // TeacherAvailibilities
    getTeacherAvailibilities: () => axiosInstance.get('/api/teacher-availibilities'),
    getTeacherAvailibilityById: (id) => axiosInstance.get(`/api/teacher-availibility/${id}`),
    createTeacherAvailibility: (data) => axiosInstance.post('/api/teacher-availibility', data),
    updateTeacherAvailibility: (id, data) => axiosInstance.put(`/api/teacher-availibility/${id}`, data),
    deleteTeacherAvailibility: (id) => axiosInstance.delete(`/api/teacher-availibility/${id}`),

     // Students 
    getStudents: () => axiosInstance.get('/api/students'),
    getStudentById: (id) => axiosInstance.get(`/api/student/${id}`),
    createStudent: (data) => axiosInstance.post('/api/student', data),
    updateStudent: (id, data) => axiosInstance.put(`/api/student/${id}`, data),
    deleteStudent: (id) => axiosInstance.delete(`/api/student/${id}`),

     // Roles 
    getRoles: (params = '') => axiosInstance.get(`/api/roles${params}`),
    getRoleById: (id) => axiosInstance.get(`/api/role/${id}`),
    createRole: (data) => axiosInstance.post('/api/role', data),
    updateRole: (id, data) => axiosInstance.put(`/api/role/${id}`, data),
    deleteRole: (id) => axiosInstance.delete(`/api/role/${id}`),

     // Permissions 
    getPermissions: (params = '') => axiosInstance.get(`/api/permissions${params}`),
    getPermissionById: (id) => axiosInstance.get(`/api/permission/${id}`),
    createPermission: (data) => axiosInstance.post('/api/permission', data),
    updatePermission: (id, data) => axiosInstance.put(`/api/permission/${id}`, data),
    deletePermission: (id) => axiosInstance.delete(`/api/permission/${id}`),

    getPositions: (params = '') => axiosInstance.get(`/api/positions${params}`),
    getPositionById: (id) => axiosInstance.get(`/api/position/${id}`),
    createPosition: (data) => axiosInstance.post('/api/position', data),
    updatePosition: (id, data) => axiosInstance.put(`/api/position/${id}`, data),
    deletePosition: (id) => axiosInstance.delete(`/api/position/${id}`),

     // Rooms 
    getRooms: () => axiosInstance.get('/api/rooms'),
    getRoomById: (id) => axiosInstance.get(`/api/room/${id}`),
    createRoom: (data) => axiosInstance.post('/api/room', data),
    updateRoom: (id, data) => axiosInstance.put(`/api/room/${id}`, data),
    deleteRoom: (id) => axiosInstance.delete(`/api/room/${id}`),

     // Subjects 
    getSubjects: () => axiosInstance.get('/api/subjects'),
    getSubjectById: (id) => axiosInstance.get(`/api/subject/${id}`),
    createSubject: (data) => axiosInstance.post('/api/subject', data),
    updateSubject: (id, data) => axiosInstance.put(`/api/subject/${id}`, data),
    deleteSubject: (id) => axiosInstance.delete(`/api/subject/${id}`),

     // AcademicYears 
    getAcademicYears: () => axiosInstance.get('/api/academic-years'),
    getAcademicYearById: (id) => axiosInstance.get(`/api/academic-year/${id}`),
    createAcademicYear: (data) => axiosInstance.post('/api/academic-year', data),
    updateAcademicYear: (id, data) => axiosInstance.put(`/api/academic-year/${id}`, data),
    deleteAcademicYear: (id) => axiosInstance.delete(`/api/academic-year/${id}`),

     // Semesters 
    getSemesters: () => axiosInstance.get('/api/semesters'),
    getSemesterById: (id) => axiosInstance.get(`/api/semester/${id}`),
    createSemester: (data) => axiosInstance.post('/api/semester', data),
    updateSemester: (id, data) => axiosInstance.put(`/api/semester/${id}`, data),
    deleteSemester: (id) => axiosInstance.delete(`/api/semester/${id}`),

     // Levels 
    getLevels: () => axiosInstance.get('/api/levels'),
    getLevelById: (id) => axiosInstance.get(`/api/level/${id}`),
    createLevel: (data) => axiosInstance.post('/api/level', data),
    updateLevel: (id, data) => axiosInstance.put(`/api/level/${id}`, data),
    deleteLevel: (id) => axiosInstance.delete(`/api/level/${id}`),

     // ClassLevels 
    getClassLevels: () => axiosInstance.get('/api/class-levels'),
    getClassLevelById: (id) => axiosInstance.get(`/api/class-level/${id}`),
    createClassLevel: (data) => axiosInstance.post('/api/class-level', data),
    updateClassLevel: (id, data) => axiosInstance.put(`/api/class-level/${id}`, data),
    deleteClassLevel: (id) => axiosInstance.delete(`/api/class-level/${id}`),


     // ClassSchedules 
    getClassSchedules: () => axiosInstance.get('/api/class-schedules'),
    getClassScheduleById: (id) => axiosInstance.get(`/api/class-schedule/${id}`),
    createClassSchedule: (data) => axiosInstance.post('/api/class-schedule', data),
    updateClassSchedule: (id, data) => axiosInstance.put(`/api/class-schedule/${id}`, data),
    deleteClassSchedule: (id) => axiosInstance.delete(`/api/class-schedule/${id}`),

     // ClassSubjects 
    getClassSubjects: () => axiosInstance.get('/api/class-subjects'),
    getClassSubjectById: (id) => axiosInstance.get(`/api/class-subject/${id}`),
    createClassSubject: (data) => axiosInstance.post('/api/class-subject', data),
    updateClassSubject: (id, data) => axiosInstance.put(`/api/class-subject/${id}`, data),
    deleteClassSubject: (id) => axiosInstance.delete(`/api/class-subject/${id}`),

     // Classs 
    getClasss: () => axiosInstance.get('/api/classs'),
    getClassById: (id) => axiosInstance.get(`/api/class/${id}`),
    createClass: (data) => axiosInstance.post('/api/class', data),
    updateClass: (id, data) => axiosInstance.put(`/api/class/${id}`, data),
    deleteClass: (id) => axiosInstance.delete(`/api/class/${id}`),

     // ReportCards 
    getReportCards: () => axiosInstance.get('/api/report-cards'),
    getReportCardById: (id) => axiosInstance.get(`/api/report-card/${id}`),
    createReportCard: (data) => axiosInstance.post('/api/report-card', data),
    updateReportCard: (id, data) => axiosInstance.put(`/api/report-card/${id}`, data),
    deleteReportCard: (id) => axiosInstance.delete(`/api/report-card/${id}`),

     // TimeSlots 
    getTimeSlots: () => axiosInstance.get('/api/time-slots'),
    getTimeSlotById: (id) => axiosInstance.get(`/api/time-slot/${id}`),
    createTimeSlot: (data) => axiosInstance.post('/api/time-slot', data),
    updateTimeSlot: (id, data) => axiosInstance.put(`/api/time-slot/${id}`, data),
    deleteTimeSlot: (id) => axiosInstance.delete(`/api/time-slot/${id}`),

     // StudentEnrollments 
    getStudentEnrollments: () => axiosInstance.get('/api/student-enrollments'),
    getStudentEnrollmentById: (id) => axiosInstance.get(`/api/student-enrollment/${id}`),
    createStudentEnrollment: (data) => axiosInstance.post('/api/student-enrollment', data),
    updateStudentEnrollment: (id, data) => axiosInstance.put(`/api/student-enrollment/${id}`, data),
    deleteStudentEnrollment: (id) => axiosInstance.delete(`/api/student-enrollment/${id}`),

     // SubjectPreferences 
    getSubjectPreferences: () => axiosInstance.get('/api/subject-preferences'),
    getSubjectPreferenceById: (id) => axiosInstance.get(`/api/subject-preference/${id}`),
    createSubjectPreference: (data) => axiosInstance.post('/api/subject-preference', data),
    updateSubjectPreference: (id, data) => axiosInstance.put(`/api/subject-preference/${id}`, data),
    deleteSubjectPreference: (id) => axiosInstance.delete(`/api/subject-preference/${id}`),

     // SubjectRoomRequirements 
    getSubjectRoomRequirements: () => axiosInstance.get('/api/subject-room-requirements'),
    getSubjectRoomRequirementById: (id) => axiosInstance.get(`/api/subject-room-requirement/${id}`),
    createSubjectRoomRequirement: (data) => axiosInstance.post('/api/subject-room-requirement', data),
    updateSubjectRoomRequirement: (id, data) => axiosInstance.put(`/api/subject-room-requirement/${id}`, data),
    deleteSubjectRoomRequirement: (id) => axiosInstance.delete(`/api/subject-room-requirement/${id}`),

    // Attendances 
    getAttendances: () => axiosInstance.get('/api/attendances'),
    getAttendanceById: (id) => axiosInstance.get(`/api/attendance/${id}`),
    createAttendance: (data) => axiosInstance.post('/api/attendance', data),
    updateAttendance: (id, data) => axiosInstance.put(`/api/attendance/${id}`, data),
    deleteAttendance: (id) => axiosInstance.delete(`/api/attendance/${id}`),
};

export default axiosInstance;