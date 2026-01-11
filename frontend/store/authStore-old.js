import { create } from 'zustand';
import { api } from '@/lib/api';

const useAuthStore = create((set) => ({
    user: null,
    accessToken: null,
    refreshToken: null,
    isAuthenticated: false,
    isLoading: false,
    error: null,

    // Initialize auth from localStorage
    initialize: () => {
        if (typeof window !== 'undefined') {
            const accessToken = localStorage.getItem('access_token');
            const refreshToken = localStorage.getItem('refresh_token');
            const userStr = localStorage.getItem('user');

            if (accessToken && refreshToken && userStr) {
                try {
                    const user = JSON.parse(userStr);
                    set({ user, accessToken, refreshToken, isAuthenticated: true });
                } catch (error) {
                    localStorage.clear();
                }
            }
        }
    },

    // Login
    login: async (email, password) => {
        set({ isLoading: true, error: null });
        try {
            const response = await api.login({ email, password });

            if (response.success && response.data) {
                const { access_token, refresh_token, user } = response.data;

                localStorage.setItem('access_token', access_token);
                localStorage.setItem('refresh_token', refresh_token);
                localStorage.setItem('user', JSON.stringify(user));

                set({
                    user,
                    accessToken: access_token,
                    refreshToken: refresh_token,
                    isAuthenticated: true,
                    isLoading: false,
                    error: null,
                });

                return { success: true };
            } else {
                set({ isLoading: false, error: response.message || 'Login failed' });
                return { success: false, error: response.message };
            }
        } catch (error) {
            const errorMessage = error.response?.data?.message || 'An error occurred';
            set({ isLoading: false, error: errorMessage });
            return { success: false, error: errorMessage };
        }
    },

    // Register
    register: async (name, email, password) => {
        set({ isLoading: true, error: null });
        try {
            const response = await api.register({ name, email, password });

            if (response.success && response.data) {
                const { access_token, refresh_token, user } = response.data;

                localStorage.setItem('access_token', access_token);
                localStorage.setItem('refresh_token', refresh_token);
                localStorage.setItem('user', JSON.stringify(user));

                set({
                    user,
                    accessToken: access_token,
                    refreshToken: refresh_token,
                    isAuthenticated: true,
                    isLoading: false,
                    error: null,
                });

                return { success: true };
            } else {
                set({ isLoading: false, error: response.message || 'Registration failed' });
                return { success: false, error: response.message };
            }
        } catch (error) {
            const errorMessage = error.response?.data?.message || 'An error occurred';
            set({ isLoading: false, error: errorMessage });
            return { success: false, error: errorMessage };
        }
    },

    // Logout
    logout: () => {
        localStorage.removeItem('access_token');
        localStorage.removeItem('refresh_token');
        localStorage.removeItem('user');
        set({
            user: null,
            accessToken: null,
            refreshToken: null,
            isAuthenticated: false,
            error: null,
        });
    },

    // Clear error
    clearError: () => set({ error: null }),
}));

export default useAuthStore;