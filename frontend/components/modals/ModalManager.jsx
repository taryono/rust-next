'use client';

import { Modal } from 'react-bootstrap';
import useModalStore from '@/store/modalStore';

// import semua modal content
import EditProfileModal from './EditProfileModal';
import DeleteConfirmModal from './DeleteConfirmModal';
import AddMemberModal from './AddMemberModal';

export default function ModalManager() {
    const { show, type, data, closeModal } = useModalStore();

    if (!type) return null;

    return (
        <Modal
            show={show}
            onHide={closeModal}
            centered
            backdrop="static"
        >
            {type === 'edit-profile' && (
                <EditProfileModal data={data} onClose={closeModal} />
            )}

            {type === 'delete-confirm' && (
                <DeleteConfirmModal data={data} onClose={closeModal} />
            )}

            {type === 'add-member' && (
                <AddMemberModal data={data} onClose={closeModal} />
            )}
        </Modal>
    );
}
