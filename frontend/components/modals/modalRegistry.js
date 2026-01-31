import dynamic from 'next/dynamic';

const modalRegistry = {
    'edit-profile': dynamic(() => import('./EditProfileModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
    'delete-confirm': dynamic(() => import('./DeleteConfirmModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
    'add-member': dynamic(() => import('./AddMemberModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
    'update-password': dynamic(() => import('./UpdatePasswordModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
    'add-role': dynamic(() => import('./AddRoleModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
    'add-permission': dynamic(() => import('./AddPermissionModal'), {
        ssr: false,
        loading: () => <div className="p-4 text-center">Loading...</div>,
    }),
};

export default modalRegistry;
