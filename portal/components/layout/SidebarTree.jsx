// frontend/components/layout/Sidebar.jsx
'use client';

import clsx from 'clsx';
import { usePathname } from 'next/navigation';
import useUIStore from '@/store/uiStore';
import useAuthStore from '@/store/authStore';
import treeMenuConfig from '@/config/tree_menu';
import TreeMenuItem from './../ui/TreeMenuItem';
import { useEffect, useState } from 'react';

export default function Sidebar() {
  const { sidebarOpen, sidebarCollapsed, closeSidebar } = useUIStore();
  const { user } = useAuthStore();
  const pathname = usePathname();
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const checkMobile = () => setIsMobile(window.innerWidth < 768);
    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  const handleLinkClick = () => {
    if (isMobile) {
      closeSidebar();
    }
  };

  const filteredMenu = treeMenuConfig.filter(
    (menu) =>
      !menu.roles ||
      (user && menu.roles.some(role => user.roles.includes(role)))
  );

  return (
    <>
      {isMobile && sidebarOpen && (
        <div 
          className="sidebar-overlay d-md-none" 
          onClick={closeSidebar}
        />
      )}

      <aside 
        className={clsx(
          'sidebar',
          isMobile ? sidebarOpen && 'show' : '',
          !isMobile && sidebarCollapsed && 'collapsed'
        )}
      >
        <div className="sidebar-header">
          <h3 className="navbar-brand mb-3">
            {sidebarCollapsed && !isMobile ? 'MA' : 'My Admin'}
          </h3>
        </div>

        <ul className="navbar-nav">
          {filteredMenu.map((menu, idx) => (
            <TreeMenuItem
              key={idx}
              menu={menu}
              menuKey={`menu-${idx}`}
              onLinkClick={handleLinkClick}
            />
          ))}
        </ul>
      </aside>
    </>
  );
}