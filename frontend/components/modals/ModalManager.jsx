'use client';

import { Modal } from 'react-bootstrap';
import useModalStore from '@/store/modalStore';
import modalRegistry from './modalRegistry';

export default function ModalManager() {
    const { show, type, data, closeModal } = useModalStore();

    if (!show || !type) return null;

    const ModalComponent = modalRegistry[type];

    if (!ModalComponent) {
        console.error(`Modal "${type}" tidak terdaftar`);
        return null;
    }

    return (
        <Modal show={show} onHide={closeModal} centered backdrop="static">
            <ModalComponent data={data} onClose={closeModal} />
        </Modal>
    );
}
