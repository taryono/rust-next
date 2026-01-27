import { create } from 'zustand';
import { api } from '@/lib/api';
import { cookies } from '@/lib/cookies';

const normalizeUser = (user) => {
  if (!user) return null;

  return {
    ...user,
    roles: Array.isArray(user.roles) ? user.roles : [],
  };
};


const useAuthStore = create((set, get) => ({
    user: { roles: [] },
    accessToken: null,
    refreshToken: null,
    isAuthenticated: false,
    isLoading: false,
    error: null,

    // Initialize auth from cookies
    initialize: () => {
        const accessToken = cookies.getAccessToken();
        const refreshToken = cookies.getRefreshToken();
        const user = cookies.getUser();

        if (accessToken && refreshToken && user) {
            set({
                user: normalizeUser(user),
                accessToken,
                refreshToken,
                isAuthenticated: true
            });
            return true;
        } else {
            set({
                user: null,
                accessToken: null,
                refreshToken: null,
                isAuthenticated: false
            });
            return false;
        }
    },

    // Login
    login: async (email, password) => {
        set({ isLoading: true, error: null });
        try {
            const response = await api.login({ email, password });
            console.log(response);
            if (response.success && response.data) {
                const { access_token, refresh_token, user } = response.data;

                // Save to cookies
                cookies.setTokens(access_token, refresh_token);
                cookies.setUser(user);

                set({
                    user: normalizeUser(user),
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
            let errorMessage = 'Terjadi kesalahan';

            // Backend mati / tidak bisa diakses
            if (
                error.code === 'ERR_NETWORK' ||
                error.message?.includes('Network Error')
            ) {
                errorMessage = 'Server sedang tidak tersedia. Silakan coba lagi nanti.';
            }
            // Timeout
            else if (error.code === 'ECONNABORTED') {
                errorMessage = 'Koneksi ke server timeout.';
            }
            // Response dari backend (401, 422, dll)
            else if (error.response?.data?.message) {
                errorMessage = error.response.data.message;
            }

            set({
                isLoading: false,
                error: errorMessage,
            });

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

                // Save to cookies
                cookies.setTokens(access_token, refresh_token);
                cookies.setUser(user);

                set({
                    user: normalizeUser(user),
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
    updateUser: (updatedUser) =>
    set((state) => ({
        user: normalizeUser({
        ...state.user,
        ...updatedUser,
        }),
    })), 
    // Logout
    logout: () => {
        // Clear cookies first
        cookies.clearAll();

        // Then clear state
        set({
            user: null,
            accessToken: null,
            refreshToken: null,
            isAuthenticated: false,
            error: null,
            isLoading: false,
        });

        // Redirect to login page
        if (typeof window !== 'undefined') {
            window.location.href = '/login';
        }
    },

    // Clear error
    clearError: () => set({ error: null }),
}));

export default useAuthStore;