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
    login: (data) => axiosInstance.post('/api/auth/login', data),
    register: (data) => axiosInstance.post('/api/auth/register', data),
    refresh: (refreshToken) => axiosInstance.post('/api/auth/refresh', { refresh_token: refreshToken }),

    // Users
    getUsers: () => axiosInstance.get('/api/users'),
    getUserById: (id) => axiosInstance.get(`/api/users/${id}`),
    getCurrentUser: () => axiosInstance.get('/api/users/me'),
    updateCurrentUser: (data) => axiosInstance.put('/api/users/me', data),
    changePassword: (data) => axiosInstance.post('/api/users/change-password', data),
    deleteUser: (id) => axiosInstance.delete(`/api/users/${id}`),
};

export default axiosInstance;