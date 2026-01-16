'use client';

import Link from 'next/link';
import clsx from 'clsx';
import { usePathname } from 'next/navigation';
import useUIStore from '@/store/uiStore';
import useAuthStore from '@/store/authStore';
import menuConfig from '@/config/menu';
import { useEffect } from 'react';

export default function Sidebar() {
  const { sidebarOpen, sidebarCollapsed, closeSidebar } = useUIStore();
  const { user } = useAuthStore();
  const pathname = usePathname();

  useEffect(() => {
    console.log(user);
    if (window.innerWidth < 768) {
      closeSidebar();
    }
  }, [closeSidebar, user]);
  const handleLinkClick = () => {
    // Close sidebar on mobile after clicking link
    if (window.innerWidth < 768) {
      closeSidebar();
    }
  };

  return (
    <>
      {/* Mobile Overlay */}
      {sidebarOpen && (
        <div 
          className="sidebar-overlay d-md-none" 
          onClick={closeSidebar}
        />
      )}

      {/* Sidebar */}
      <aside 
        className={clsx(
          'sidebar',
          sidebarOpen && 'show',
          sidebarCollapsed && 'collapsed'
        )}
      >
        <div className="sidebar-header">
          <h3 className="navbar-brand mb-3">
            {sidebarCollapsed ? 'MA' : 'My Admin'}
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
                  {!sidebarCollapsed && <span>{menu.label}</span>}
                </Link>
              </li>
            ))}
        </ul>
      </aside>
    </>
  );
}