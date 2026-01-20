'use client';
 
import clsx from 'clsx';
import Navbar from './Navbar';
// import Sidebar from './Sidebar';
import SidebarTree from './SidebarTree';
import useUIStore from '@/store/uiStore'; 
export default function AuthLayout({ children }) {
  const { sidebarCollapsed } = useUIStore();

  return (
    <>
      <Navbar />
      <div className="layout-container">
        <SidebarTree />
        <main className={clsx('main-content', sidebarCollapsed && 'collapsed')}>
          {children}
        </main>
      </div>
    </>
  );
}