'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';

export default function AddMemberModal({ data, onClose }) {
    const { updateUser } = useAuthStore();

    const [form, setForm] = useState({
        name: '',
        email: '',
        dob: '',// date of birth select datepicker
        pob: '', // place of birth 
        password: '',
        role: 'member', // select member options 
        status: 'inactive',
        image: '', // input file upload
        bio: '',
        phone: '',
        address: '',
        city: '', // select city dropdown
        province: '', // select province dropdown
        country: '', // select country dropdown
        postal_code: '',
        latitude: '', 
        longitude: '',
        timezone: '', 

    });

    useEffect(() => {
        if (data) {
            setForm({
                name: data.name,
                email: data.email,
                password: '',
                role: 'member',
                status: 'inactive',
                image: '',
                bio: '',
                phone: '',
                address: '',
                city: '',
                province: '',
                country: '',
                postal_code: '',
                latitude: '',
                longitude: '',
                timezone: '',
            });
        }
    }, [data]);

    const handleSubmit = async () => {
        try {
            const res = await api.createMember(form);

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
