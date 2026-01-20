'use client';

import useUIStore from '@/store/uiStore';
import UserDropdown from './UserDropdown';
import { useState, useEffect } from 'react';

export default function Navbar() {
  const { toggleSidebar } = useUIStore(); // ✅ Nama yang benar
  const [isMobile, setIsMobile] = useState(false);

  useEffect(() => {
    const checkMobile = () => {
      setIsMobile(window.innerWidth < 768);
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);
    return () => window.removeEventListener('resize', checkMobile);
  }, []);

  const handleToggle = () => {
    console.log('🍔 Toggle clicked!');
    toggleSidebar();
  };
  return (
    <header className="navbar navbar-light bg-light shadow-sm">
      <div className="container-fluid d-flex justify-content-between align-items-center">
        
        {/* Left: Hamburger */}
        <div className='d-flex justify-content-between  align-items-center'> 
          {/* Desktop Toggle Button */}
          {!isMobile && (
            <button
              className="btn btn-icon d-none d-md-inline-flex"
              onClick={toggleSidebar}
              aria-label="Toggle sidebar collapse"
            >
              <i className="ti ti-menu-2"></i>
            </button>
          )}
          {isMobile && (
          <button
            className="btn btn-outline-secondary"
            onClick={handleToggle} // ✅ Function yang benar
          >
            <i className="bi bi-list"></i>
          </button>
          )}
          {!isMobile && (
          <div className="d-flex align-items-center ms-2">
              <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="blue" className="bi bi-diagram-3 me-3" viewBox="0 0 16 16">
                  <path fillRule="evenodd" d="M6 3.5A1.5 1.5 0 0 1 7.5 2h1A1.5 1.5 0 0 1 10 3.5v1A1.5 1.5 0 0 1 8.5 6v1H14a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-1 0V8h-5v.5a.5.5 0 0 1-1 0V8h-5v.5a.5.5 0 0 1-1 0v-1A.5.5 0 0 1 2 7h5.5V6A1.5 1.5 0 0 1 6 4.5v-1zM8.5 5a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1zM0 11.5A1.5 1.5 0 0 1 1.5 10h1A1.5 1.5 0 0 1 4 11.5v1A1.5 1.5 0 0 1 2.5 14h-1A1.5 1.5 0 0 1 0 12.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1zm4.5.5A1.5 1.5 0 0 1 7.5 10h1a1.5 1.5 0 0 1 1.5 1.5v1A1.5 1.5 0 0 1 8.5 14h-1A1.5 1.5 0 0 1 6 12.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1zm4.5.5a1.5 1.5 0 0 1 1.5-1.5h1a1.5 1.5 0 0 1 1.5 1.5v1a1.5 1.5 0 0 1-1.5 1.5h-1a1.5 1.5 0 0 1-1.5-1.5v-1zm1.5-.5a.5.5 0 0 0-.5.5v1a.5.5 0 0 0 .5.5h1a.5.5 0 0 0 .5-.5v-1a.5.5 0 0 0-.5-.5h-1z" />
              </svg>
              <h3 className="text-white mb-0 fw-bold text-primary">Silsilah App</h3>
          </div>
          )}
        </div>
        {/* Right: User */}
        <UserDropdown />
      </div>
    </header>
  );
}
 