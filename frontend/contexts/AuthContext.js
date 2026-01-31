// frontend/contexts/AuthContext.js
'use client';

import { createContext, useContext, useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { cookies } from '@/lib/cookies';
const AuthContext = createContext();

export function AuthProvider({ children }) {
  const router = useRouter();
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Check token on mount
    const savedToken = cookies.getAccessToken();
    const savedUser = cookies.getUser();
    
    if (savedToken && savedUser) {
      setToken(savedToken);
      setUser(savedUser);
    }
    
    setLoading(false);
  }, []);

  const login = (userData, authToken, refresh_token) => {
    setUser(userData);
    setToken(authToken);
    cookies.setTokens(authToken,refresh_token);
    cookies.setUser(userData) 
  };

  const logout = () => {
    setUser(null);
    setToken(null);
    cookies.clearAll(); 
    router.push('/login');
  };

  // Function to handle unauthorized errors
  const handleUnauthorized = () => {
    console.log('🚫 Token expired - logging out...');
    logout();
  };

  return (
    <AuthContext.Provider value={{ 
      user, 
      token, 
      loading, 
      login, 
      logout, 
      handleUnauthorized 
    }}>
      {children}
    </AuthContext.Provider>
  );
}

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};