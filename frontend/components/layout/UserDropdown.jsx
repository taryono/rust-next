'use client';

import useAuthStore from '@/store/authStore';
import useModalStore from '@/store/modalStore';

export default function UserDropdown() {
  const { user, logout } = useAuthStore();
  const { openModal } = useModalStore();
  if (!user) return null;

  return (
    <>
    <span className="text-white">
        <i className="bi bi-person-circle me-2"></i>
        {user.name}
    </span>
    <div className="dropdown">
      <button
        className="btn btn-dark dropdown-toggle"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        <i className="bi bi-person-circle me-2"></i>
        
      </button>

      <ul className="dropdown-menu dropdown-menu-end">
        <li className="dropdown-item text-muted">
          Login as <strong>{user.name}</strong>          
        </li>
        <li><hr className="dropdown-divider" /></li> 
        <li><button className="dropdown-item" onClick={() => openModal('update-password', user)}>Reset Password</button></li>
        <li><hr className="dropdown-divider" /></li>
        <li>
          <button className="dropdown-item text-danger" onClick={logout}>
            Logout
          </button>
        </li>
      </ul>
    </div>
    </>
  );
}
