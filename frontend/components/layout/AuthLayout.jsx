'use client';
 
import clsx from 'clsx';
import Navbar from './Navbar';
import Sidebar from './Sidebar';
import useUIStore from '@/store/uiStore';
import { useEffect } from 'react';
import useAuthStore from '@/store/authStore';

export default function AuthLayout({ children }) {
  const { sidebarCollapsed } = useUIStore();
  const {me} = useAuthStore();

  useEffect (() => {
    if (me) {
      console.log(me);
    }
  }, [me]);

  return (
    <>
      <Navbar />
      <div className="layout-container">
        <Sidebar />
        <main className={clsx('main-content', sidebarCollapsed && 'collapsed')}>
          {children}
        </main>
      </div>
    </>
  );
}