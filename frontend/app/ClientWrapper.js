// frontend\app\ClientWrapper.js
'use client';

import { useEffect } from "react";
import useAuthStore from '@/store/authStore';
import { AuthProvider } from '@/contexts/AuthContext';
import BootstrapClient from '@/components/ui/BootstrapClient';
import ModalManager from '@/components/modals/ModalManager';

export default function ClientWrapper({ children }) {
  const initialize = useAuthStore(s => s.initialize);

  useEffect(() => {
    initialize();
  }, []);

  return (
    <AuthProvider>
      {children}
      <ModalManager />
      <BootstrapClient />
    </AuthProvider>
  );
}