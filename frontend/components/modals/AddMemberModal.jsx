'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import { useAuth } from '@/contexts/AuthContext';
import { api } from '@/lib/api';
import { alertSuccess, alertError } from '@/lib/alert';

export default function AddMemberModal({ data, onClose, onSuccess }) {
    const { user } = useAuth();
    const [imagePreview, setImagePreview] = useState(null);
    const [errors, setErrors] = useState({});
    const [loading, setLoading] = useState(false);

    const [form, setForm] = useState({
        name: 'Muhammad Syahdan Ksatria',
        email: 'muhammad.syahdan@gmail.com',
        dob: '2020-01-12', // ✅ Format YYYY-MM-DD
        pob: 'Brebes',
        password: 'password',
        password_confirm: 'password',
        role: 'teacher',
        status: '1',
        image: null,
        bio: 'Syahdan anak yang cerdas,soleh,hebat,sehat',
        phone: '087883732016',
        address: 'slatri Utara RT/RW: 001/003',
        city: 'Brebes',
        province: 'Jawa Tengah',
        country: 'Indonesia',
        postal_code: '52262',
        latitude: '-6.9175',
        longitude: '107.9161',
        timezone: 'Asia/Jakarta'
    });

    useEffect(() => {
        if (data) {
            setForm({
                name: data.name || '',
                email: data.email || '',
                dob: data.dob || '',
                pob: data.pob || '',
                password: '',
                password_confirm: '',
                role: data.role || 'member',
                status: data.status || 'active',
                image: null,
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
            // Validasi ukuran file (max 2MB)
            if (file.size > 2 * 1024 * 1024) {
                alertError('Ukuran file maksimal 2MB');
                return;
            }

            // Validasi tipe file
            if (!file.type.startsWith('image/')) {
                alertError('File harus berupa gambar');
                return;
            }

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
        
        // Validasi nama
        if (!form.name.trim()) {
            newErrors.name = 'Nama wajib diisi';
        } else if (form.name.trim().length < 3) {
            newErrors.name = 'Nama minimal 3 karakter';
        }

        // Validasi email
        if (!form.email.trim()) {
            newErrors.email = 'Email wajib diisi';
        } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(form.email)) {
            newErrors.email = 'Format email tidak valid';
        }
        
        // Validasi password (hanya untuk user baru)
        if (!data) {
            if (!form.password) {
                newErrors.password = 'Password wajib diisi';
            } else if (form.password.length < 6) {
                newErrors.password = 'Password minimal 6 karakter';
            }
            
            if (form.password !== form.password_confirm) {
                newErrors.password_confirm = 'Password tidak cocok';
            }
        } else {
            // Untuk edit, validasi password hanya jika diisi
            if (form.password && form.password.length < 6) {
                newErrors.password = 'Password minimal 6 karakter';
            }
            
            if (form.password && form.password !== form.password_confirm) {
                newErrors.password_confirm = 'Password tidak cocok';
            }
        }

        // Validasi phone (optional tapi harus valid jika diisi)
        if (form.phone && !/^[0-9+\-\s()]*$/.test(form.phone)) {
            newErrors.phone = 'Format nomor telepon tidak valid';
        }
        
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    };

    const handleSubmit = async () => {
        if (!validateForm()) {
            alertError('Mohon periksa kembali form Anda');
            return;
        }

        setLoading(true);
        
        try {
            // Jika ada image, gunakan FormData
            let dataToSend;
            
            if (form.image instanceof File) {
                const formData = new FormData();
                
                // Append semua field ke FormData
                formData.append('name', form.name);
                formData.append('email', form.email);
                formData.append('password', form.password);
                formData.append('role', form.role);
                formData.append('status', form.status);
                formData.append('foundation_id', user.foundation_id);
                
                // Append optional fields jika ada
                if (form.dob) formData.append('dob', form.dob);
                if (form.pob) formData.append('pob', form.pob);
                if (form.bio) formData.append('bio', form.bio);
                if (form.phone) formData.append('phone', form.phone);
                if (form.address) formData.append('address', form.address);
                if (form.city) formData.append('city', form.city);
                if (form.province) formData.append('province', form.province);
                if (form.country) formData.append('country', form.country);
                if (form.postal_code) formData.append('postal_code', form.postal_code);
                if (form.latitude) formData.append('latitude', form.latitude);
                if (form.longitude) formData.append('longitude', form.longitude);
                if (form.timezone) formData.append('timezone', form.timezone);
                
                // Append image
                formData.append('image', form.image);
                
                dataToSend = formData;
            } else {
                // Jika tidak ada image, gunakan JSON biasa
                dataToSend = {
                    name: form.name,
                    email: form.email,
                    password: form.password,
                    role: form.role,
                    status: form.status,
                    foundation_id: user.foundation_id,
                    dob: form.dob || null,
                    pob: form.pob || null,
                    bio: form.bio || null,
                    phone: form.phone || null,
                    address: form.address || null,
                    city: form.city || null,
                    province: form.province || null,
                    country: form.country || 'Indonesia',
                    postal_code: form.postal_code || null,
                    latitude: form.latitude || null,
                    longitude: form.longitude || null,
                    timezone: form.timezone || 'Asia/Jakarta',
                };
                
                // Hapus password jika kosong (untuk edit)
                if (data && !form.password) {
                    delete dataToSend.password;
                }
            }
            
            let response;
            if (data) {
                // Update user
                response = await api.updateUser(data.id, dataToSend);
                alertSuccess('User berhasil diupdate!');
            } else {
                // Create user
                response = await api.createUser(dataToSend);
                alertSuccess('User berhasil ditambahkan!');
            }
            
            console.log('Response:', response);
            
            // Callback success
            if (onSuccess) {
                onSuccess(response);
            }
            
            // Close modal
            onClose();
            
        } catch (err) {
            console.error('Error submit:', err);
            
            // Handle specific error messages
            if (err.response?.data?.message) {
                alertError(err.response.data.message);
            } else if (err.response?.status === 409) {
                alertError('Email sudah terdaftar');
            } else if (err.response?.status === 422) {
                alertError('Data tidak valid, periksa kembali form Anda');
            } else {
                alertError('Gagal menyimpan data: ' + (err.message || 'Unknown error'));
            }
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
                        <small className="text-muted mt-1">Max 2MB, format: JPG, PNG, GIF</small>
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
                        className={`form-control ${errors.phone ? 'is-invalid' : ''}`}
                        placeholder="+62 812-3456-7890"
                        value={form.phone}
                        onChange={(e) => setForm({ ...form, phone: e.target.value })}
                    />
                    {errors.phone && <div className="invalid-feedback">{errors.phone}</div>}
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
                            <option value="teacher">Teacher</option>
                            <option value="student">Student</option>
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