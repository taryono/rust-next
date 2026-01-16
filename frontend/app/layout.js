'use client';

import { useEffect } from "react";
import useAuthStore from '@/store/authStore';
import 'bootstrap/dist/css/bootstrap.min.css';
import '@tabler/core/dist/css/tabler.min.css';
import 'bootstrap-icons/font/bootstrap-icons.css';
import "./globals.css";
import BootstrapClient from '@/components/ui/BootstrapClient';
import ModalManager from '@/components/modals/ModalManager';
export default function RootLayout({ children }) {

  const initialize = useAuthStore(state => state.initialize);

  useEffect(() => {
    initialize();
  }, []);

  return (
    <html lang="en">
      <body>
        {children}
        <ModalManager /> 

        {/* Bootstrap JS */}
        <BootstrapClient />
        
      </body>
      
    </html>
  );
}
