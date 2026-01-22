'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';
import { toastSuccess, toastError } from '@/lib/toast';  

export default function UpdatePasswordModal({ data, onClose }) {
    const {updateUser, initialize } = useAuthStore();
    
     const [form, setForm] = useState({
        new_password: '', 
        confirm_password:'',
        old_password:''
    }); 
     
    const handleSubmit = async () => {
        try { 
            if(form.new_password.length < 6 && form.confirm_password.length < 6 && form.old_password.length < 6){
                toastError('Password minimal 6 karakter');
                return;
            }
            if(form.new_password !== form.confirm_password){
                toastError('Password dan Ulangi Password harus sama');
                return;
            }
            let res = await api.changePassword(form);

            // UPDATE STATE FRONTEND
            await updateUser(res.data);

            // optional: refresh user di store
            await initialize();
            toastSuccess('Update Password berhasil diperbarui');
            // hide modal
            onClose();
        } catch (error) {
            console.error('Update Password gagal:', error);
            toastError('Update Password gagal :'+error.message);
            onClose();
        }
    };

    return (
        <>
            <Modal.Header closeButton>
                <Modal.Title>Edit Password</Modal.Title>
            </Modal.Header>

            <Modal.Body>
                <label className='label'>Password Saat Ini</label>
                <input
                    className="form-control mb-3"
                    placeholder="Old Password"
                    value={form.old_password}
                    type="password"
                    onChange={(e) =>
                        setForm({ ...form, old_password: e.target.value })
                    }
                /> 
                <label className='label'>Password Baru</label>
                <input
                    className="form-control mb-3"
                    placeholder="Password"
                    value={form.new_password}
                    type="password"
                    onChange={(e) =>
                        setForm({ ...form, new_password: e.target.value })
                    }
                /> 
                <label className='label'>Ulangi Password</label>
                <input
                    className="form-control mb-3"
                    placeholder="confirm Password"
                    value={form.confirm_password}
                    type="password"
                    onChange={(e) =>
                        setForm({ ...form, confirm_password: e.target.value })
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
