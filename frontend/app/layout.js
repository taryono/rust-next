'use client';

import { useEffect } from "react";
import "./globals.css";
import useAuthStore from '@/store/authStore';
import 'bootstrap/dist/css/bootstrap.min.css';
import 'bootstrap-icons/font/bootstrap-icons.css';

export default function RootLayout({ children }) {

  const initialize = useAuthStore(state => state.initialize);

  useEffect(() => {
    initialize();
  }, []);

  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
