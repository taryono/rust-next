// frontend/components/layout/Sidebar.jsx
'use client';

import Link from 'next/link';
import clsx from 'clsx';
import { usePathname } from 'next/navigation';
import useUIStore from '@/store/uiStore';
import useAuthStore from '@/store/authStore';
import menuConfig from '@/config/menu';
import { useEffect, useState } from 'react';
// import css 
import "@/app/sidebar.css";

export default function Sidebar() {
  const { sidebarOpen, sidebarCollapsed, closeSidebar } = useUIStore();
  const { user } = useAuthStore();
  const pathname = usePathname();
  const [isMobile, setIsMobile] = useState(false);

  // Detect mobile on mount and resize
  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkMobile(); // Check on mount
    window.addEventListener('resize', checkMobile);

    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  useEffect(() => {
    console.log('User:', user);
    console.log('Is Mobile:', isMobile);
    console.log('Sidebar Open:', sidebarOpen);
    console.log('Sidebar Collapsed:', sidebarCollapsed);
  }, [user, isMobile, sidebarOpen, sidebarCollapsed]);

  const handleLinkClick = () => {
    // Close sidebar on mobile after clicking link
    if (isMobile) {
      closeSidebar();
    }
  };

  return (
    <>
      {/* Mobile Overlay - hanya tampil di mobile */}
      {isMobile && sidebarOpen && (
        <div 
          className="sidebar-overlay d-md-none" 
          onClick={closeSidebar}
        />
      )}

      {/* Sidebar */}
      <aside 
        className={clsx(
          'sidebar',
          isMobile ? sidebarOpen && 'show' : '', // Mobile: show/hide
          !isMobile && sidebarCollapsed && 'collapsed' // Desktop: collapsed/expanded
        )}
      >
        <div className="sidebar-header">
          <h3 className="navbar-brand mb-3">
            {sidebarCollapsed && !isMobile ? 'MA' : 'My Admin'}
          </h3>
        </div>

        <ul className="navbar-nav">
          {menuConfig
            .filter(
              (menu) =>
                !menu.roles ||
                (user && menu.roles.some(role => user.roles.includes(role)))
            )
            .map((menu) => (
              <li 
                key={menu.href} 
                className={clsx('nav-item', pathname === menu.href && 'active')}
              >
                <Link 
                  href={menu.href} 
                  className="nav-link" 
                  onClick={handleLinkClick}
                >
                  <i className={menu.icon}></i>
                  {(!sidebarCollapsed || isMobile) && <span>{menu.label}</span>}
                </Link>
              </li>
            ))}
        </ul>
      </aside>
    </>
  );
}