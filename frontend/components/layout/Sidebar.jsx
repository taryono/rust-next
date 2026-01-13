'use client';

import Link from 'next/link';
import clsx from 'clsx';
import useUIStore from '@/store/uiStore';
import useAuthStore from '@/store/authStore';
import menuConfig from '@/config/menu';
import { usePathname } from 'next/navigation';

export default function Sidebar() {
  const { sidebarOpen,closeSidebar  } = useUIStore();
  const { user } = useAuthStore();
  const pathname = usePathname();
  return (
    // <aside
    //   className={clsx( 'navbar navbar-vertical navbar-dark bg-dark',  sidebarOpen && 'show' )}
    //   style={{ minHeight: '100vh' }}
    // >
    <aside className={`sidebar ${sidebarOpen ? 'show' : ''}`}> 
      <div className="container-fluid">
        <h3 className="navbar-brand mb-3">My Admin</h3>
        <ul className="navbar-nav">
          {menuConfig
            .filter((menu) => !menu.roles || (user && menu.roles.includes(user.role)))
            .map((menu) => (
              <li key={menu.href} className={clsx('nav-item',pathname === menu.href && 'active')}>
                <Link href={menu.href} className="nav-link" onClick={closeSidebar}>
                  {menu.label}
                  <i className={menu.icon}></i>
                </Link>
              </li>
            ))}
        </ul>
      </div>
    </aside>
  );
}
