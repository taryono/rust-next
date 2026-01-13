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
};

export default modalRegistry;
