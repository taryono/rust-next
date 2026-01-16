export default [
  {
    label: 'Dashboard',
    href: '/dashboard',
    icon: 'bi-house',
    menuContext: 'global',
  },
  {
    label: 'Users',
    href: '/dashboard/users',
    icon: 'bi-people',
    roles: ['system_owner'],
    menuContext: 'system_owner',
  },
  {
    label: 'Reports',
    href: '/reports',
    icon: 'bi-bar-chart',
    roles: ['admin', 'staff'],
    menuContext: 'reports',
  },
];
