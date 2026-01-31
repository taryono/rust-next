// frontend/config/menu.js
const treeMenuConfig = [
  {
    key:'dashboard',
    label: 'Dashboard',
    href: '/dashboard',
    icon: 'ti ti-dashboard',
    menuContext: 'global',
  },
  {
  key:'parent_user',
  label: 'Authentications',
  icon: 'ti ti-users',
  roles: ['system_owner', 'admin'],
  menuContext: 'system_owner',
  children: [
    { 
      key:'user',
      label: 'User',
      href: '/dashboard/users',
      icon: 'ti ti-list',
      permissions: ['users.view']
    }, 
    {
      key:'role',
      label: 'Role',
      href: '/dashboard/roles',
      icon: 'ti ti-shield',
      permissions: ['roles.view']
    },
    {
      key:'permission',
      label: 'Permission',
      href: '/dashboard/permissions',
      icon: 'ti ti-key',
      permissions: ['permissions.view']
    },
    {
      key:'role_permission',
      label: 'Role Permissions',
      href: '/dashboard/role-permissions',
      icon: 'ti ti-link',
      permissions: ['role-permissions.view']
    }
  ]
},
  {
    key:'foundation',
    label: 'Foundation',
    icon: 'ti ti-users',
    roles: ['system_owner'],
    menuContext: 'system_owner',
    children: [
      {
        key:'unit',
        label: 'Unit',
        href: '/dashboard/units',
        icon: 'ti ti-list'
      }, 
      {
        key:'bisnis',
        label: 'Bisnis Units', 
        icon: 'ti ti-shield',
        children: [
            {
                key:'loundry',
                label: 'Loundry',
                href: '/dashboard/loundry',
                icon: 'ti ti-list'
            }, 
            {
                key:'parking',
                label: 'Parking',
                href: '/dashboard/parking_areas',
                icon: 'ti ti-shield',
                
            }, 
            {
                key:'canteen',
                label: 'Canteen',
                href: '/dashboard/canteens',
                icon: 'ti ti-shield',
                
            }
        ]
      }
    ]
  },
  {
    key:'parent_employee',
    label: 'HR',
    icon: 'ti ti-package',
    roles: ['system_owner'],
    menuContext: 'system_owner',
    children: [
      {
        key:'position',
        label: 'Positions',
        href: '/dashboard/positions',
        icon: 'ti ti-list'
      },
      {
        key:'employee',
        key:'dashboard',
        label: 'Employee',
        href: '/dashboard/employees',
        icon: 'ti ti-list'
      },
      {
        key:'status_employee',
        label: 'Employee',
        icon: 'ti ti-category',
        children: [
          {
            key:'permanent',
            label: 'Permanent Employee',
            href: '/dashboard/employees/permanent',
          },
          {
            key:'contract',
            label: 'Contract Employee',
            href: '/dashboard/employees/contract',
          },
          {
            key:'freelance',
            label: 'Freelancer Employee',
            href: '/dashboard/employees/freelance',
          }
        ]
      }
    ]
  },
  {
    key:'setting',
    label: 'Settings',
    href: '/dashboard/settings',
    icon: 'ti ti-settings',
    roles: ['system_owner'],
    menuContext: 'system_owner',
  }
];

export default treeMenuConfig;