'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import { studentApi } from '@/lib/api/studentApi';
import { toastSuccess, toastError } from '@/lib/toast';

// ============================================================
// Props:
//   mode       -> 'add' | 'edit'
//   data       -> object (isi saat edit, null saat add)
//   onClose    -> callback tutup modal
//   onRefresh  -> callback setelah berhasil create / update
// ============================================================
export default function StudentForm({ mode = 'add', data = null, onClose, onRefresh }) {
    const isEdit = mode === 'edit';

    const [form, setForm] = useState({
        user_id: '',
        name: '',
        foundation_id: '',
        unit_id: '',
        student_number: '',
        class_id: '',
        parent_name: '',
        parent_phone: '',
        enrollment_date: '',
        graduation_date: '',
    });

    const [loading, setLoading] = useState(false);
    const [errors, setErrors] = useState({});

    // ========== Populate form saat Edit ==========
    useEffect(() => {
        if (isEdit && data) {
            setForm({
                user_id: data.user_id ?? '',
                name: data.name ?? '',
                foundation_id: data.foundation_id ?? '',
                unit_id: data.unit_id ?? '',
                student_number: data.student_number ?? '',
                class_id: data.class_id ?? '',
                parent_name: data.parent_name ?? '',
                parent_phone: data.parent_phone ?? '',
                enrollment_date: data.enrollment_date
                    ? formatDatetimeLocal(data.enrollment_date)
                    : '',
                graduation_date: data.graduation_date
                    ? formatDatetimeLocal(data.graduation_date)
                    : '',
            });
        }
    }, [data, isEdit]);

    // ========== Helper: format ISO -> datetime-local ==========
    const formatDatetimeLocal = (isoString) => {
        if (!isoString) return '';
        // potong timezone info agar cocok dengan input type="datetime-local"
        return new Date(isoString).toISOString().slice(0, 16);
    };

    // ========== Generic change handler ==========
    const handleChange = (e) => {
        const { name, value } = e.target;
        setForm((prev) => ({ ...prev, [name]: value }));
        // hapus error saat user mulai mengetik
        if (errors[name]) {
            setErrors((prev) => ({ ...prev, [name]: '' }));
        }
    };

    // ========== Client-side validation ==========
    const validate = () => {
        const errs = {};
        if (!form.user_id) errs.user_id = 'User ID wajib diisi';
        if (!form.name.trim()) errs.name = 'Nama wajib diisi';
        if (!form.foundation_id) errs.foundation_id = 'Foundation ID wajib diisi';
        if (!form.student_number.trim()) errs.student_number = 'Nomor siswa wajib diisi';
        if (form.parent_phone && !/^[0-9+\-() ]+$/.test(form.parent_phone)) {
            errs.parent_phone = 'Format telepon tidak valid';
        }
        setErrors(errs);
        return Object.keys(errs).length === 0;
    };

    // ========== Payload builder: buang field kosong opsional ==========
    const buildPayload = () => {
        const payload = {
            user_id: Number(form.user_id),
            name: form.name.trim(),
            foundation_id: Number(form.foundation_id),
            student_number: form.student_number.trim(),
        };

        // Optional fields — kirim null kalau kosong
        if (form.unit_id !== '') payload.unit_id = Number(form.unit_id);
        if (form.class_id !== '') payload.class_id = Number(form.class_id);
        if (form.parent_name !== '') payload.parent_name = form.parent_name.trim();
        if (form.parent_phone !== '') payload.parent_phone = form.parent_phone.trim();
        if (form.enrollment_date !== '') payload.enrollment_date = new Date(form.enrollment_date).toISOString();
        if (form.graduation_date !== '') payload.graduation_date = new Date(form.graduation_date).toISOString();

        return payload;
    };

    // ========== Submit ==========
    const handleSubmit = async () => {
        if (!validate()) return;

        setLoading(true);
        try {
            const payload = buildPayload();

            if (isEdit) {
                await studentApi.update(data.id, payload);
                toastSuccess('Data siswa berhasil diperbarui');
            } else {
                await studentApi.create(payload);
                toastSuccess('Data siswa berhasil ditambahkan');
            }

            onRefresh?.(); // refresh tabel
            onClose?.();   // tutup modal
        } catch (error) {
            console.error('Student form error:', error);
            toastError((isEdit ? 'Update' : 'Tambah') + ' siswa gagal: ' + error.message);
        } finally {
            setLoading(false);
        }
    };

    // ========== Render helper: input + error ==========
    const InputField = ({ label, name, type = 'text', placeholder = '', required = false, className = '' }) => (
        <div className={`mb-3 ${className}`}>
            <label className="form-label">
                {label}
                {required && <span className="text-danger"> *</span>}
            </label>
            <input
                type={type}
                name={name}
                className={`form-control ${errors[name] ? 'is-invalid' : ''}`}
                placeholder={placeholder}
                value={form[name]}
                onChange={handleChange}
                disabled={loading}
            />
            {errors[name] && (
                <div className="invalid-feedback">{errors[name]}</div>
            )}
        </div>
    );

    return (
        <>
            {/* ===== HEADER ===== */}
            <Modal.Header closeButton>
                <Modal.Title>
                    {isEdit ? '✎ Edit Siswa' : '+ Tambah Siswa'}
                </Modal.Title>
            </Modal.Header>

            {/* ===== BODY ===== */}
            <Modal.Body>
                {/* --- Row 1: Nama & Nomor Siswa --- */}
                <div className="row">
                    <div className="col-md-7">
                        <InputField
                            label="Nama Siswa"
                            name="name"
                            placeholder="Nama lengkap siswa"
                            required
                        />
                    </div>
                    <div className="col-md-5">
                        <InputField
                            label="Nomor Siswa"
                            name="student_number"
                            placeholder="ex: STU-2024-001"
                            required
                        />
                    </div>
                </div>

                {/* --- Row 2: User ID & Foundation ID --- */}
                <div className="row">
                    <div className="col-md-6">
                        <InputField
                            label="User ID"
                            name="user_id"
                            type="number"
                            placeholder="ID linked user"
                            required
                        />
                    </div>
                    <div className="col-md-6">
                        <InputField
                            label="Foundation ID"
                            name="foundation_id"
                            type="number"
                            placeholder="ID yayasan"
                            required
                        />
                    </div>
                </div>

                {/* --- Row 3: Unit ID & Class ID (optional) --- */}
                <div className="row">
                    <div className="col-md-6">
                        <InputField
                            label="Unit ID"
                            name="unit_id"
                            type="number"
                            placeholder="ID unit (opsional)"
                        />
                    </div>
                    <div className="col-md-6">
                        <InputField
                            label="Class ID"
                            name="class_id"
                            type="number"
                            placeholder="ID kelas (opsional)"
                        />
                    </div>
                </div>

                {/* --- Row 4: Orang Tua (optional) --- */}
                <div className="row">
                    <div className="col-md-6">
                        <InputField
                            label="Nama Orang Tua"
                            name="parent_name"
                            placeholder="Nama orang tua (opsional)"
                        />
                    </div>
                    <div className="col-md-6">
                        <InputField
                            label="Telepon Orang Tua"
                            name="parent_phone"
                            placeholder="ex: 08xx-xxxx-xxxx"
                        />
                    </div>
                </div>

                {/* --- Row 5: Tanggal (optional) --- */}
                <div className="row">
                    <div className="col-md-6">
                        <InputField
                            label="Tanggal Masuk"
                            name="enrollment_date"
                            type="datetime-local"
                        />
                    </div>
                    <div className="col-md-6">
                        <InputField
                            label="Tanggal Lulus"
                            name="graduation_date"
                            type="datetime-local"
                        />
                    </div>
                </div>
            </Modal.Body>

            {/* ===== FOOTER ===== */}
            <Modal.Footer>
                <button
                    className="btn btn-secondary"
                    onClick={onClose}
                    disabled={loading}
                >
                    Cancel
                </button>
                <button
                    className="btn btn-primary"
                    onClick={handleSubmit}
                    disabled={loading}
                >
                    {loading
                        ? (isEdit ? 'Menyimpan...' : 'Menambahkan...')
                        : (isEdit ? 'Simpan Perubahan' : 'Tambah Siswa')}
                </button>
            </Modal.Footer>
        </>
    );
}
