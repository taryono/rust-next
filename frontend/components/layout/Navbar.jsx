'use client';

import useUIStore from '@/store/uiStore';
import UserDropdown from './UserDropdown';

export default function Navbar() {
  const { toggleSidebar } = useUIStore(); // ✅ Nama yang benar
  const handleToggle = () => {
    console.log('🍔 Toggle clicked!');
    toggleSidebar();
  };
  return (
    <header className="navbar navbar-light bg-light shadow-sm">
      <div className="container-fluid d-flex justify-content-between align-items-center">
        
        {/* Left: Hamburger */}
        <button
          className="btn btn-outline-secondary"
          onClick={handleToggle} // ✅ Function yang benar
        >
          <i className="bi bi-list"></i>
        </button>

        {/* Right: User */}
        <UserDropdown />
      </div>
    </header>
  );
}