'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';
import { toastSuccess, toastError } from '@/lib/toast';



export default function AddPermissionModal({ data, onClose }) {
    const { initialize } = useAuthStore();

    const [form, setForm] = useState({
        name: '', 
    });

    useEffect(() => {
        if (data) {
            setForm({
                name: data.name, 
            });
        }
    }, [data]); 

    const handleSubmit = async () => {
        try { 
            let res = await api.createPermission(form); 
            toastSuccess('Permission berhasil diperbarui');
            // hide modal
            onClose();
        } catch (error) {
            console.error('Update berhasil gagal:', error);
            toastError('Update berhasil gagal :'+error.message);
            onClose();

        }
    };

    return (
        <>
            <Modal.Header closeButton>
                <Modal.Title>Add Permission</Modal.Title>
            </Modal.Header>

            <Modal.Body>
                <input
                    className="form-control mb-3"
                    placeholder="Name"
                    value={form.name}
                    onChange={(e) =>
                        setForm({ ...form, name: e.target.value })
                    }
                /> 
            </Modal.Body>

            <Modal.Footer>
                <button className="btn btn-secondary" onClick={onClose}>
                    Cancel
                </button>
                <button className="btn btn-primary" onClick={handleSubmit}>
                    Save
                </button>
            </Modal.Footer>
        </>
    );
}
