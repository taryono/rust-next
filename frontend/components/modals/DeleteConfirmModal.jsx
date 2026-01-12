'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';

export default function EditProfileModal({ data, onClose }) {
    const { updateUser } = useAuthStore();

    const [form, setForm] = useState({
        name: '',
        email: '',
        password: '',
    });

    useEffect(() => {
        if (data) {
            setForm({
                name: data.name,
                email: data.email,
                password: '',
            });
        }
    }, [data]);

    const handleSubmit = async () => {
        try {
            const res = await fetch(
                `${process.env.NEXT_PUBLIC_API_URL}/profile`,
                {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json',
                        Authorization: `Bearer ${localStorage.getItem('access_token')}`,
                    },
                    body: JSON.stringify(form),
                }
            );

            const updatedUser = await res.json();

            updateUser(updatedUser);
            onClose();
        } catch (err) {
            console.error(err);
            alert('Update profile gagal');
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
                <input
                    type="password"
                    className="form-control"
                    placeholder="Password (optional)"
                    value={form.password}
                    onChange={(e) =>
                        setForm({ ...form, password: e.target.value })
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
