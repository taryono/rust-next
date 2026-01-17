'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import useAuthStore from '@/store/authStore';
import { api } from '@/lib/api';

export default function AddMemberModal({ data, onClose }) {
    const { updateUser } = useAuthStore();
    const [imagePreview, setImagePreview] = useState(null);
    const [errors, setErrors] = useState({});
    const [loading, setLoading] = useState(false);

    const [form, setForm] = useState({
        name: '',
        email: '',
        dob: '',
        pob: '',
        password: '',
        password_confirm: '',
        role: 'member',
        status: 'inactive',
        image: null,
        bio: '',
        phone: '',
        address: '',
        city: '',
        province: '',
        country: 'Indonesia',
        postal_code: '',
        latitude: '',
        longitude: '',
        timezone: 'Asia/Jakarta',
    });

    useEffect(() => {
        if (data) {
            setForm({
                ...form,
                name: data.name || '',
                email: data.email || '',
                dob: data.dob || '',
                pob: data.pob || '',
                role: data.role || 'member',
                status: data.status || 'inactive',
                bio: data.bio || '',
                phone: data.phone || '',
                address: data.address || '',
                city: data.city || '',
                province: data.province || '',
                country: data.country || 'Indonesia',
                postal_code: data.postal_code || '',
                latitude: data.latitude || '',
                longitude: data.longitude || '',
                timezone: data.timezone || 'Asia/Jakarta',
            });
            if (data.image) {
                setImagePreview(data.image);
            }
        }
    }, [data]);

    const handleImageChange = (e) => {
        const file = e.target.files[0];
        if (file) {
            setForm({ ...form, image: file });
            const reader = new FileReader();
            reader.onloadend = () => {
                setImagePreview(reader.result);
            };
            reader.readAsDataURL(file);
        }
    };

    const validateForm = () => {
        const newErrors = {};
        
        if (!form.name.trim()) newErrors.name = 'Nama wajib diisi';
        if (!form.email.trim()) newErrors.email = 'Email wajib diisi';
        if (!form.email.includes('@')) newErrors.email = 'Email tidak valid';
        
        if (!data) { // Hanya validasi password untuk user baru
            if (!form.password) newErrors.password = 'Password wajib diisi';
            if (form.password && form.password.length < 6) {
                newErrors.password = 'Password minimal 6 karakter';
            }
            if (form.password !== form.password_confirm) {
                newErrors.password_confirm = 'Password tidak cocok';
            }
        }
        
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    };

    const handleSubmit = async () => {
        if (!validateForm()) return;

        setLoading(true);
        try {
            const formData = new FormData();
            Object.keys(form).forEach(key => {
                if (key === 'image' && form.image instanceof File) {
                    formData.append(key, form.image);
                } else if (form[key]) {
                    formData.append(key, form[key]);
                }
            });

            const res = await api.createMember(formData);
            const updatedUser = await res.json();
            updateUser(updatedUser);
            onClose();
        } catch (err) {
            console.error(err);
            alert('Gagal menambahkan member: ' + (err.message || 'Unknown error'));
        } finally {
            setLoading(false);
        }
    };

    return (
        <>
            <Modal.Header closeButton>
                <Modal.Title>
                    {data ? 'Edit Member' : 'Tambah Member Baru'}
                </Modal.Title>
            </Modal.Header>

            <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}>
                {/* Photo Upload Section */}
                <div className="mb-4 text-center">
                    <label className="form-label fw-bold">Foto Profil</label>
                    <div className="d-flex flex-column align-items-center">
                        <div className="avatar avatar-xl mb-3" style={{ width: '120px', height: '120px' }}>
                            {imagePreview ? (
                                <img 
                                    src={imagePreview} 
                                    alt="Preview" 
                                    className="rounded-circle"
                                    style={{ width: '100%', height: '100%', objectFit: 'cover' }}
                                />
                            ) : (
                                <div className="avatar-placeholder rounded-circle bg-secondary d-flex align-items-center justify-content-center" style={{ width: '100%', height: '100%' }}>
                                    <i className="bi bi-person" style={{ fontSize: '3rem' }}></i>
                                </div>
                            )}
                        </div>
                        <input
                            type="file"
                            className="form-control w-auto"
                            accept="image/*"
                            onChange={handleImageChange}
                        />
                    </div>
                </div>

                <hr className="my-4" />

                {/* Basic Information */}
                <h5 className="mb-3 text-primary">
                    <i className="bi bi-person-badge me-2"></i>Informasi Dasar
                </h5>
                
                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">
                            Nama Lengkap <span className="text-danger">*</span>
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
                            Email <span className="text-danger">*</span>
                        </label>
                        <input
                            type="email"
                            className={`form-control ${errors.email ? 'is-invalid' : ''}`}
                            placeholder="email@example.com"
                            value={form.email}
                            onChange={(e) => setForm({ ...form, email: e.target.value })}
                        />
                        {errors.email && <div className="invalid-feedback">{errors.email}</div>}
                    </div>
                </div>

                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">Tanggal Lahir</label>
                        <input
                            type="date"
                            className="form-control"
                            value={form.dob}
                            onChange={(e) => setForm({ ...form, dob: e.target.value })}
                        />
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">Tempat Lahir</label>
                        <input
                            className="form-control"
                            placeholder="Kota/Kabupaten"
                            value={form.pob}
                            onChange={(e) => setForm({ ...form, pob: e.target.value })}
                        />
                    </div>
                </div>

                <div className="mb-3">
                    <label className="form-label">Nomor Telepon</label>
                    <input
                        type="tel"
                        className="form-control"
                        placeholder="+62 812-3456-7890"
                        value={form.phone}
                        onChange={(e) => setForm({ ...form, phone: e.target.value })}
                    />
                </div>

                <div className="mb-3">
                    <label className="form-label">Bio</label>
                    <textarea
                        className="form-control"
                        rows="3"
                        placeholder="Ceritakan tentang diri Anda..."
                        value={form.bio}
                        onChange={(e) => setForm({ ...form, bio: e.target.value })}
                    />
                </div>

                <hr className="my-4" />

                {/* Account Settings */}
                <h5 className="mb-3 text-primary">
                    <i className="bi bi-shield-lock me-2"></i>Pengaturan Akun
                </h5>

                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">
                            Password {!data && <span className="text-danger">*</span>}
                        </label>
                        <input
                            type="password"
                            className={`form-control ${errors.password ? 'is-invalid' : ''}`}
                            placeholder={data ? 'Kosongkan jika tidak ingin mengubah' : 'Minimal 6 karakter'}
                            value={form.password}
                            onChange={(e) => setForm({ ...form, password: e.target.value })}
                        />
                        {errors.password && <div className="invalid-feedback">{errors.password}</div>}
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">
                            Konfirmasi Password {!data && <span className="text-danger">*</span>}
                        </label>
                        <input
                            type="password"
                            className={`form-control ${errors.password_confirm ? 'is-invalid' : ''}`}
                            placeholder="Ulangi password"
                            value={form.password_confirm}
                            onChange={(e) => setForm({ ...form, password_confirm: e.target.value })}
                        />
                        {errors.password_confirm && <div className="invalid-feedback">{errors.password_confirm}</div>}
                    </div>
                </div>

                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">Role</label>
                        <select
                            className="form-select"
                            value={form.role}
                            onChange={(e) => setForm({ ...form, role: e.target.value })}
                        >
                            <option value="member">Member</option>
                            <option value="admin">Admin</option>
                            <option value="moderator">Moderator</option>
                        </select>
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">Status</label>
                        <select
                            className="form-select"
                            value={form.status}
                            onChange={(e) => setForm({ ...form, status: e.target.value })}
                        >
                            <option value="active">Active</option>
                            <option value="inactive">Inactive</option>
                            <option value="pending">Pending</option>
                        </select>
                    </div>
                </div>

                <hr className="my-4" />

                {/* Address Information */}
                <h5 className="mb-3 text-primary">
                    <i className="bi bi-geo-alt me-2"></i>Informasi Alamat
                </h5>

                <div className="mb-3">
                    <label className="form-label">Alamat Lengkap</label>
                    <textarea
                        className="form-control"
                        rows="2"
                        placeholder="Jalan, RT/RW, Kelurahan/Desa"
                        value={form.address}
                        onChange={(e) => setForm({ ...form, address: e.target.value })}
                    />
                </div>

                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">Kota/Kabupaten</label>
                        <input
                            className="form-control"
                            placeholder="Contoh: Bandung"
                            value={form.city}
                            onChange={(e) => setForm({ ...form, city: e.target.value })}
                        />
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">Provinsi</label>
                        <select
                            className="form-select"
                            value={form.province}
                            onChange={(e) => setForm({ ...form, province: e.target.value })}
                        >
                            <option value="">Pilih Provinsi</option>
                            <option value="Jawa Barat">Jawa Barat</option>
                            <option value="Jawa Tengah">Jawa Tengah</option>
                            <option value="Jawa Timur">Jawa Timur</option>
                            <option value="DKI Jakarta">DKI Jakarta</option>
                            <option value="Banten">Banten</option>
                            <option value="Sumatera Utara">Sumatera Utara</option>
                            <option value="Sumatera Barat">Sumatera Barat</option>
                            <option value="Sumatera Selatan">Sumatera Selatan</option>
                            <option value="Bali">Bali</option>
                            <option value="Kalimantan Timur">Kalimantan Timur</option>
                        </select>
                    </div>
                </div>

                <div className="row mb-3">
                    <div className="col-md-6 mb-3">
                        <label className="form-label">Negara</label>
                        <select
                            className="form-select"
                            value={form.country}
                            onChange={(e) => setForm({ ...form, country: e.target.value })}
                        >
                            <option value="Indonesia">Indonesia</option>
                            <option value="Malaysia">Malaysia</option>
                            <option value="Singapore">Singapore</option>
                        </select>
                    </div>

                    <div className="col-md-6 mb-3">
                        <label className="form-label">Kode Pos</label>
                        <input
                            className="form-control"
                            placeholder="40xxx"
                            value={form.postal_code}
                            onChange={(e) => setForm({ ...form, postal_code: e.target.value })}
                        />
                    </div>
                </div>

                <div className="row mb-3">
                    <div className="col-md-4 mb-3">
                        <label className="form-label">Latitude</label>
                        <input
                            type="number"
                            step="any"
                            className="form-control"
                            placeholder="-6.9175"
                            value={form.latitude}
                            onChange={(e) => setForm({ ...form, latitude: e.target.value })}
                        />
                    </div>

                    <div className="col-md-4 mb-3">
                        <label className="form-label">Longitude</label>
                        <input
                            type="number"
                            step="any"
                            className="form-control"
                            placeholder="107.6191"
                            value={form.longitude}
                            onChange={(e) => setForm({ ...form, longitude: e.target.value })}
                        />
                    </div>

                    <div className="col-md-4 mb-3">
                        <label className="form-label">Timezone</label>
                        <select
                            className="form-select"
                            value={form.timezone}
                            onChange={(e) => setForm({ ...form, timezone: e.target.value })}
                        >
                            <option value="Asia/Jakarta">WIB (Jakarta)</option>
                            <option value="Asia/Makassar">WITA (Makassar)</option>
                            <option value="Asia/Jayapura">WIT (Jayapura)</option>
                        </select>
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