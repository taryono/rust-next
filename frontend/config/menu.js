export default [
  {
    label: 'Dashboard',
    href: '/dashboard',
    icon: 'bi-house',
  },
  {
    label: 'Users',
    href: '/users',
    icon: 'bi-people',
    roles: ['admin'],
  },
  {
    label: 'Reports',
    href: '/reports',
    icon: 'bi-bar-chart',
    roles: ['admin', 'staff'],
  },
];
