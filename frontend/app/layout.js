'use client';

import { useEffect } from "react";
import useAuthStore from '@/store/authStore'; 
import 'bootstrap-icons/font/bootstrap-icons.css';
import '@tabler/core/dist/css/tabler.min.css';
import "./globals.css";
import BootstrapClient from '@/components/ui/BootstrapClient';
import ModalManager from '@/components/modals/ModalManager';
import { AuthProvider } from '@/contexts/AuthContext';
export default function RootLayout({ children }) {

  const initialize = useAuthStore(state => state.initialize);

  useEffect(() => {
    initialize();
  }, []);

  return (
    <html lang="en">
      <body>
       <AuthProvider>
 
        {children}
        <ModalManager /> 

        {/* Bootstrap JS */}
        <BootstrapClient />
        </AuthProvider>
      </body>
      
    </html>
  );
}
