'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';
import { toastSuccess, toastError } from '@/lib/toast';



export default function EditProfileModal({ data, onClose }) {
    const { updateUser, initialize } = useAuthStore();

    const [form, setForm] = useState({
        name: '',
        email: '', 
    });

    useEffect(() => {
        if (data) {
            setForm({
                name: data.name,
                email: data.email, 
            });
        }
    }, [data]); 

    const handleSubmit = async () => {
        try { 
            let res = await api.updateCurrentUser(form);

            // UPDATE STATE FRONTEND
            await updateUser(res.data);

            // optional: refresh user di store
            await initialize();
            toastSuccess('Profile berhasil diperbarui');
            // hide modal
            onClose();
        } catch (error) {
            console.error('Update profile gagal:', error);
            toastError('Update profile gagal :'+error.message);
            onClose();

        }
    };

    return (
        <>
            <Modal.Header closeButton>
                <Modal.Title>Edit Profile</Modal.Title>
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
                <input
                    className="form-control mb-3"
                    placeholder="Email"
                    value={form.email}
                    onChange={(e) =>
                        setForm({ ...form, email: e.target.value })
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
