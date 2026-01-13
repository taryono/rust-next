'use client';

import useAuthStore from '@/store/authStore';

export default function UserDropdown() {
  const { user, logout } = useAuthStore();

  if (!user) return null;

  return (
    <div className="dropdown">
      <button
        className="btn btn-dark dropdown-toggle"
        data-bs-toggle="dropdown"
        aria-expanded="false"
      >
        <i className="bi bi-person-circle me-2"></i>
        {user.name}
      </button>

      <ul className="dropdown-menu dropdown-menu-end">
        <li className="dropdown-item text-muted">
          Login as <strong>{user.email}</strong>
        </li>
        <li><hr className="dropdown-divider" /></li>
        <li><button className="dropdown-item">Update Account</button></li>
        <li><button className="dropdown-item">Reset Password</button></li>
        <li><hr className="dropdown-divider" /></li>
        <li>
          <button className="dropdown-item text-danger" onClick={logout}>
            Logout
          </button>
        </li>
      </ul>
    </div>
  );
}
