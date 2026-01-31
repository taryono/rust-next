// frontend/lib/cookies.js
import Cookies from 'js-cookie';

const TOKEN_KEY = 'access_token';
const REFRESH_TOKEN_KEY = 'refresh_token';
const USER_KEY = 'user';

export const cookies = {
    // Set tokens with expiry
    setTokens: (accessToken, refreshToken) => {
        const cookieOptions = {
            path: '/',
            sameSite: 'strict',
        };

        // Access token expires in 15 minutes
        Cookies.set(TOKEN_KEY, accessToken, {
            ...cookieOptions,
            expires: 1 / 96 // 1/96 day = 15 minutes
        });

        // Refresh token expires in 7 days
        Cookies.set(REFRESH_TOKEN_KEY, refreshToken, {
            ...cookieOptions,
            expires: 7
        });
    },

    // Set user data
    setUser: (user) => {
        Cookies.set(USER_KEY, JSON.stringify(user), {
            expires: 7,
            path: '/',
            sameSite: 'strict',
        });
    },

    // Get tokens
    getAccessToken: () => {
        return Cookies.get(TOKEN_KEY);
    },

    getRefreshToken: () => {
        return Cookies.get(REFRESH_TOKEN_KEY);
    },

    // Get user
    getUser: () => {
        const userStr = Cookies.get(USER_KEY);
        try {
            return userStr ? JSON.parse(userStr) : null;
        } catch (e) {
            return null;
        }
    },

    // Remove all
    clearAll: () => {
        const cookieOptions = {
            path: '/',
        };

        Cookies.remove(TOKEN_KEY, cookieOptions);
        Cookies.remove(REFRESH_TOKEN_KEY, cookieOptions);
        Cookies.remove(USER_KEY, cookieOptions);

        // Fallback: clear dari semua possible paths
        Cookies.remove(TOKEN_KEY);
        Cookies.remove(REFRESH_TOKEN_KEY);
        Cookies.remove(USER_KEY);
    },

    // Check if authenticated
    isAuthenticated: () => {
        return !!Cookies.get(TOKEN_KEY) && !!Cookies.get(REFRESH_TOKEN_KEY);
    },
};