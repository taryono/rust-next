// frontend/config/menu.js
const treeMenuConfig = [
  {
    key: 'dashboard',
    label: 'Dashboard',
    href: '/dashboard',
    icon: 'ti ti-dashboard',
    menuContext: 'global',
  },

  // Manajemen semua yayasan (hanya system_owner)
  {
    key: 'foundations',
    label: 'Foundations',
    icon: 'ti ti-building',
    menuContext: 'system_owner',
    children: [
      { key: 'foundation_list', label: 'All Foundations', href: '/dashboard/foundations', icon: 'ti ti-list' },
      { key: 'foundation_units', label: 'Units', href: '/dashboard/foundations/units', icon: 'ti ti-building-community' },
    ]
  },

  // Auth & Access Control
  {
    key: 'auth',
    label: 'Authentications',
    icon: 'ti ti-users',
    menuContext: 'system_owner',
    children: [
      { key: 'user', label: 'User', href: '/dashboard/users', icon: 'ti ti-list' },
      { key: 'role', label: 'Role', href: '/dashboard/roles', icon: 'ti ti-shield' },
      { key: 'permission', label: 'Permission', href: '/dashboard/permissions', icon: 'ti ti-key' },
      { key: 'role_permission', label: 'Role Permissions', href: '/dashboard/role-permissions', icon: 'ti ti-link' },
    ]
  },

  // HR - dibedakan karyawan yayasan vs guru
  {
    key: 'hr',
    label: 'HR',
    icon: 'ti ti-package',
    menuContext: 'system_owner',
    children: [
      { key: 'position', label: 'Jabatan', href: '/dashboard/positions', icon: 'ti ti-list' },
      {
        key: 'foundation_employee',
        label: 'Karyawan Yayasan',
        icon: 'ti ti-users',
        children: [
          { key: 'permanent',icon: 'ti ti-package', label: 'Permanent', href: '/dashboard/employees/permanent' },
          { key: 'contract',icon: 'ti ti-package', label: 'Contract', href: '/dashboard/employees/contract' },
          { key: 'honorary',icon: 'ti ti-package', label: 'Honorer', href: '/dashboard/employees/honorary' },
        ]
      },
      {
        key: 'teacher',
        label: 'Guru (Per Unit)',
        icon: 'ti ti-school',
        children: [
          // ini idealnya dinamis dari API berdasarkan unit yang ada
          { key: 'teacher_list',icon: 'ti ti-package', label: 'All Teachers', href: '/dashboard/teachers' },
        ]
      }
    ]
  },

  {
    key: 'departments',
    label: 'Departments',
    icon: 'ti ti-package',
    menuContext: 'system_owner',
    href: '/dashboard/departments',
  },
  {
    key: 'foundation_types',
    label: 'Tipe Yayasan',
    icon: 'ti ti-package',
    menuContext: 'system_owner',
    href: '/dashboard/foundation_types',
  },

  {
    key: 'setting',
    label: 'Settings',
    href: '/dashboard/settings',
    icon: 'ti ti-settings',
    menuContext: 'system_owner',
  }
];
export default treeMenuConfig;