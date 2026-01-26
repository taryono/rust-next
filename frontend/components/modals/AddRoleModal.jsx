'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';

export default function AddRoleModal({ data, onClose }) {
    const { updateUser } = useAuthStore();
    const [imagePreview, setImagePreview] = useState(null);
    const [errors, setErrors] = useState({});
    const [loading, setLoading] = useState(false);

    const [form, setForm] = useState({
        name: '',
        description: '',
        
    });

    useEffect(() => {
        if (data) {
            setForm({
                ...form,
                name: data.name || '',
                description: data.description || '',
                 
            }); 
        }
    }, [data]);
 
    const validateForm = () => {
        const newErrors = {};
        
        if (!form.name.trim()) newErrors.name = 'Nama wajib diisi';  
        
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    };

    const handleSubmit = async () => {
        if (!validateForm()) return;

        setLoading(true);
        try {
            const formData = new FormData();
            Object.keys(form).forEach(key => { 
                formData.append(key, form[key]);
            });

            const res = await api.createRole(formData);
            const role = await res.json(); 
            onClose();
        } catch (err) {
            console.error(err);
            alert('Gagal menambahkan Role: ' + (err.message || 'Unknown error'));
        } finally {
            setLoading(false);
        }
    };

    return (
        <>
            <Modal.Header closeButton>
                <Modal.Title>
                    {data ? 'Edit Role' : 'Tambah Role Baru'}
                </Modal.Title>
            </Modal.Header>

            <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}> 
                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">
                            Nama Role <span className="text-danger">*</span>
                        </label>
                        <input
                            className={`form-control ${errors.name ? 'is-invalid' : ''}`}
                            placeholder="Masukkan nama lengkap"
                            value={form.name}
                            onChange={(e) => setForm({ ...form, name: e.target.value })}
                        />
                        {errors.name && <div className="invalid-feedback">{errors.name}</div>}
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">
                            Deskripsi <span className="text-danger">*</span>
                        </label>
                        <input
                            type="text"
                            className={`form-control ${errors.description ? 'is-invalid' : ''}`}
                            placeholder="description@example.com"
                            value={form.description}
                            onChange={(e) => setForm({ ...form, description: e.target.value })}
                        />
                        {errors.description && <div className="invalid-feedback">{errors.description}</div>}
                    </div>
                </div>
  
            </Modal.Body>

            <Modal.Footer>
                <button 
                    className="btn btn-secondary" 
                    onClick={onClose}
                    disabled={loading}
                >
                    <i className="bi bi-x-circle me-2"></i>Batal
                </button>
                <button 
                    className="btn btn-primary" 
                    onClick={handleSubmit}
                    disabled={loading}
                >
                    {loading ? (
                        <>
                            <span className="spinner-border spinner-border-sm me-2"></span>
                            Menyimpan...
                        </>
                    ) : (
                        <>
                            <i className="bi bi-check-circle me-2"></i>
                            {data ? 'Update' : 'Simpan'}
                        </>
                    )}
                </button>
            </Modal.Footer>
        </>
    );
}