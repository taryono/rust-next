'use client';

import { createContext, useContext, useEffect, useState } from 'react';
import useAuthStore from '@/store/authStore';

const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  const store = useAuthStore();

  const [ready, setReady] = useState(false); 
  // 👉 INISIALISASI SEKALI SAJA
  useEffect(() => {
    let mounted = true;

    async function init() {
      try {
        await store.initialize();
      } finally {
        if (mounted) setReady(true);
      }
    }

    init();

    return () => {
      mounted = false;
    };
  }, []);
  

  const value = {
    ...store,
    isReady: ready,   // ← flag penting
  };

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  );
}

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used in AuthProvider');
  return ctx;
};