// frontend/store/authStore.js
import { create } from 'zustand';
import { api } from '@/lib/api';
import { cookies } from '@/lib/cookies';

/* ========================================================
   UTILITIES
======================================================== */

const normalizeUser = (user) => {
  if (!user || typeof user !== 'object') return null;

  return {
    id: user.id ?? null,
    name: user.name ?? '',
    foundation_id:user.foundation_id ?? '',
    email: user.email ?? '',
    roles: Array.isArray(user.roles) ? user.roles : [],
    permissions: Array.isArray(user.permissions) ? user.permissions : [],
    meta: user.meta ?? {},
  };
};

const initialState = {
  user: null,
  accessToken: null,
  refreshToken: null,
  isAuthenticated: false,
  isLoading: false,
  isInitialized: false,
  error: null,
};

/* ========================================================
   STORE
======================================================== */

const useAuthStore = create((set, get) => ({
  ...initialState,

  /* ---------------------------------------------
     INITIALIZE (NON BLOCKING & SAFE)
  --------------------------------------------- */
  initialize: () => {
    try {
      const accessToken = cookies.getAccessToken();
      const refreshToken = cookies.getRefreshToken();
      const user = cookies.getUser();

      if (!accessToken || !refreshToken || !user) {
        set({ ...initialState, isInitialized: true });
        return false;
      }

      set({
        user: normalizeUser(user),
        accessToken,
        refreshToken,
        isAuthenticated: true,
        isInitialized: true,
      });

      return true;

    } catch (err) {
      console.warn('Auth initialize failed:', err);
      set({ ...initialState, isInitialized: true });
      return false;
    }
  },

  /* ---------------------------------------------
     LOGIN (DEFENSIVE & CLEAN)
  --------------------------------------------- */
  login: async (email, password) => {
    if (get().isLoading) return { success: false };

    set({ isLoading: true, error: null });

    try {
      const response = await api.login({ email, password });

      if (!response?.success || !response?.data) {
        throw new Error(response?.message || 'Login gagal');
      }

      const { access_token, refresh_token, user } = response.data;

      // Persist
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

    } catch (error) {
      const message = parseAuthError(error);

      set({
        isLoading: false,
        error: message,
      });

      return { success: false, error: message };
    }
  },

  /* ---------------------------------------------
     REGISTER
  --------------------------------------------- */
  register: async (name, email, password) => {
    if (get().isLoading) return { success: false };

    set({ isLoading: true, error: null });

    try {
      const response = await api.register({ name, email, password });

      if (!response?.success || !response?.data) {
        throw new Error(response?.message || 'Register gagal');
      }

      const { access_token, refresh_token, user } = response.data;

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

    } catch (error) {
      const message = parseAuthError(error);

      set({
        isLoading: false,
        error: message,
      });

      return { success: false, error: message };
    }
  },

  /* ---------------------------------------------
     REFRESH TOKEN (OPTIONAL TAPI PENTING)
  --------------------------------------------- */
  refresh: async () => {
    try {
      const refreshToken = get().refreshToken;
      if (!refreshToken) throw new Error('No refresh token');

      const response = await api.refresh(refreshToken);

      if (!response?.success) throw new Error('Refresh gagal');

      const { access_token } = response.data;

      cookies.setTokens(access_token, refreshToken);

      set({
        accessToken: access_token,
        isAuthenticated: true,
      });

      return true;

    } catch {
      get().forceLogout();
      return false;
    }
  },

  /* ---------------------------------------------
     UPDATE USER PARTIAL
  --------------------------------------------- */
  updateUser: (updated) =>
    set((state) => ({
      user: normalizeUser({
        ...(state.user || {}),
        ...updated,
      }),
    })),

  /* ---------------------------------------------
     LOGOUT (SAFE)
  --------------------------------------------- */
  logout: () => {
    cookies.clearAll();

    set({
      ...initialState,
      isInitialized: true,
    });
  },

  /* ---------------------------------------------
     FORCE LOGOUT (TOKEN EXPIRED)
  --------------------------------------------- */
  forceLogout: () => {
    cookies.clearAll();

    set({
      ...initialState,
      isInitialized: true,
      error: 'Sesi telah berakhir, silakan login ulang.',
    });
  },

  clearError: () => set({ error: null }),
}));

/* ========================================================
   HELPER ERROR PARSER
======================================================== */

function parseAuthError(error) {
  // Network mati
  if (
    error.code === 'ERR_NETWORK' ||
    error.message?.includes('Network Error')
  ) {
    return 'Server sedang tidak tersedia. Silakan coba lagi nanti.';
  }

  // Timeout
  if (error.code === 'ECONNABORTED') {
    return 'Koneksi ke server timeout.';
  }

  // Backend message
  if (error.response?.data?.message) {
    return error.response.data.message;
  }

  return error.message || 'Terjadi kesalahan tidak dikenal';
}

export default useAuthStore;