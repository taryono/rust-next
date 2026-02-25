'use client';

import { useState, useEffect, useCallback } from 'react';
import { Modal } from 'react-bootstrap';
import { useForm } from 'react-hook-form';
import { api } from '@/lib/api';
import { alertSuccess, alertError } from '@/lib/alert';
import FormSection from '@/components/ui/FormSection';

// =====================================================
// CONSTANTS
// =====================================================
const FOUNDATION_TYPES = [
  { value: 'yayasan', label: 'Yayasan' },
  { value: 'lembaga', label: 'Lembaga' },
  { value: 'organisasi', label: 'Organisasi' },
  { value: 'komunitas', label: 'Komunitas' },
];

const PROVINCES = [
  'Aceh', 'Sumatera Utara', 'Sumatera Barat', 'Riau', 'Kepulauan Riau',
  'Jambi', 'Sumatera Selatan', 'Kepulauan Bangka Belitung', 'Bengkulu', 'Lampung',
  'DKI Jakarta', 'Jawa Barat', 'Banten', 'Jawa Tengah', 'DI Yogyakarta',
  'Jawa Timur', 'Bali', 'Nusa Tenggara Barat', 'Nusa Tenggara Timur',
  'Kalimantan Barat', 'Kalimantan Tengah', 'Kalimantan Selatan',
  'Kalimantan Timur', 'Kalimantan Utara',
  'Sulawesi Utara', 'Sulawesi Tengah', 'Sulawesi Selatan',
  'Sulawesi Tenggara', 'Gorontalo', 'Sulawesi Barat',
  'Maluku', 'Maluku Utara', 'Papua', 'Papua Barat',
];

const FORM_DEFAULTS = {
  name: '',
  code: '',
  foundation_type: '',
  parent_id: '',
  address: '',
  description: '',
  city: '',
  province: '',
  phone: '',
  email: '',
  is_active: '1',
};

// =====================================================
// SUB-COMPONENTS
// =====================================================
const FormInput = ({ label, error, children, required = false }) => (
  <div className="mb-3">
    <label className="form-label fw-semibold">
      {label} {required && <span className="text-danger">*</span>}
    </label>
    {children}
    {error && <div className="invalid-feedback d-block">{error.message}</div>}
  </div>
);

// =====================================================
// MAIN COMPONENT
// =====================================================
export default function AddFoundationModal({ data, onClose, onSuccess }) {
  const [loading, setLoading] = useState(false);
  const [foundations, setFoundations] = useState([]);
  const isEdit = !!data;

  const {
    register,
    handleSubmit,
    setValue,
    formState: { errors },
  } = useForm({
    defaultValues: FORM_DEFAULTS,
    mode: 'onBlur',
  });

  // =====================================================
  // EFFECTS
  // =====================================================

  // Fetch parent foundations for dropdown
  useEffect(() => {
    const fetchFoundations = async () => {
      try {
        const response = await api.getFoundations?.();
        if (response?.data) setFoundations(response.data);
      } catch (err) {
        console.error('Failed to fetch foundations:', err);
      }
    };
    fetchFoundations();
  }, []);

  // Populate form in edit mode
  useEffect(() => {
    if (data) {
      Object.keys(FORM_DEFAULTS).forEach((key) => {
        if (data[key] !== undefined) {
          setValue(key, data[key] ?? FORM_DEFAULTS[key]);
        }
      });
    }
  }, [data, setValue]);

  // =====================================================
  // SUBMIT HANDLER
  // =====================================================
  const onSubmit = async (formData) => {
    setLoading(true);
    try {
      const payload = preparePayload(formData);

      const response = isEdit
        ? await api.updateFoundation(data.id, payload)
        : await api.createFoundation(payload);

      alertSuccess(isEdit ? 'Foundation berhasil diperbarui' : 'Foundation berhasil ditambahkan');
      onSuccess?.(response);
      onClose();
    } catch (err) {
      const errorMessage =
        err.response?.data?.message ||
        `Gagal ${isEdit ? 'mengupdate' : 'menambahkan'} foundation`;
      alertError(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  const preparePayload = (formData) => {
    const payload = { ...formData };

    // Convert empty strings to null for optional fields
    const optionalFields = ['code', 'foundation_type', 'parent_id', 'address', 'description', 'city', 'province', 'phone', 'email'];
    optionalFields.forEach((key) => {
      if (payload[key] === '' || payload[key] === undefined) {
        payload[key] = null;
      }
    });

    // Convert is_active to number
    payload.is_active = payload.is_active !== '' ? Number(payload.is_active) : null;

    // Convert parent_id to number or null
    payload.parent_id = payload.parent_id ? Number(payload.parent_id) : null;

    return payload;
  };

  // =====================================================
  // RENDER
  // =====================================================
  return (
    <>
      <Modal.Header closeButton>
        <Modal.Title>
          <i className="bi bi-building me-2"></i>
          {isEdit ? 'Edit Foundation' : 'Tambah Foundation Baru'}
        </Modal.Title>
      </Modal.Header>

      <form onSubmit={handleSubmit(onSubmit)}>
        <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}>

          {/* ── Bagian 1: Informasi Utama ── */}
          <FormSection title="Informasi Utama" icon="bi bi-building">
            <div className="row">
              <div className="col-md-8">
                <FormInput label="Nama Foundation" error={errors.name} required>
                  <input
                    className={`form-control ${errors.name ? 'is-invalid' : ''}`}
                    placeholder="Contoh: Yayasan Pendidikan Nusantara"
                    {...register('name', {
                      required: 'Nama foundation wajib diisi',
                      minLength: { value: 3, message: 'Minimal 3 karakter' },
                    })}
                  />
                </FormInput>
              </div>

              <div className="col-md-4">
                <FormInput label="Kode" error={errors.code}>
                  <input
                    className={`form-control ${errors.code ? 'is-invalid' : ''}`}
                    placeholder="Contoh: YPN-001"
                    {...register('code', {
                      maxLength: { value: 50, message: 'Maksimal 50 karakter' },
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <div className="row">
              <div className="col-md-6">
                <FormInput label="Tipe Foundation" error={errors.foundation_type}>
                  <select
                    className={`form-select ${errors.foundation_type ? 'is-invalid' : ''}`}
                    {...register('foundation_type')}
                  >
                    <option value="">-- Pilih Tipe --</option>
                    {FOUNDATION_TYPES.map((t) => (
                      <option key={t.value} value={t.value}>
                        {t.label}
                      </option>
                    ))}
                  </select>
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Parent Foundation" error={errors.parent_id}>
                  <select
                    className={`form-select ${errors.parent_id ? 'is-invalid' : ''}`}
                    {...register('parent_id')}
                  >
                    <option value="">-- Tidak Ada (Root) --</option>
                    {foundations
                      .filter((f) => !isEdit || f.id !== data?.id) // Prevent self-reference
                      .map((f) => (
                        <option key={f.id} value={f.id}>
                          {f.name}
                        </option>
                      ))}
                  </select>
                </FormInput>
              </div>
            </div>

            <FormInput label="Deskripsi" error={errors.description}>
              <textarea
                className={`form-control ${errors.description ? 'is-invalid' : ''}`}
                placeholder="Deskripsi singkat tentang foundation"
                rows="3"
                {...register('description', {
                  maxLength: { value: 1000, message: 'Maksimal 1000 karakter' },
                })}
              />
            </FormInput>
          </FormSection>

          <hr className="my-4" />

          {/* ── Bagian 2: Kontak ── */}
          <FormSection title="Informasi Kontak" icon="bi bi-telephone">
            <div className="row">
              <div className="col-md-6">
                <FormInput label="Email" error={errors.email}>
                  <input
                    type="email"
                    className={`form-control ${errors.email ? 'is-invalid' : ''}`}
                    placeholder="email@foundation.org"
                    {...register('email', {
                      pattern: {
                        value: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
                        message: 'Format email tidak valid',
                      },
                    })}
                  />
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Nomor Telepon" error={errors.phone}>
                  <input
                    className={`form-control ${errors.phone ? 'is-invalid' : ''}`}
                    placeholder="02xxxxxxxxxx"
                    {...register('phone', {
                      pattern: {
                        value: /^[0-9]{8,15}$/,
                        message: 'Nomor telepon tidak valid (8-15 digit)',
                      },
                    })}
                  />
                </FormInput>
              </div>
            </div>
          </FormSection>

          <hr className="my-4" />

          {/* ── Bagian 3: Alamat ── */}
          <FormSection title="Informasi Alamat" icon="bi bi-geo-alt">
            <FormInput label="Alamat Lengkap" error={errors.address}>
              <textarea
                rows="2"
                className={`form-control ${errors.address ? 'is-invalid' : ''}`}
                placeholder="Jalan, Nomor, RT/RW, Kelurahan/Desa"
                {...register('address')}
              />
            </FormInput>

            <div className="row">
              <div className="col-md-6">
                <FormInput label="Kota/Kabupaten" error={errors.city}>
                  <input
                    className={`form-control ${errors.city ? 'is-invalid' : ''}`}
                    placeholder="Contoh: Bandung"
                    {...register('city')}
                  />
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Provinsi" error={errors.province}>
                  <select
                    className={`form-select ${errors.province ? 'is-invalid' : ''}`}
                    {...register('province')}
                  >
                    <option value="">-- Pilih Provinsi --</option>
                    {PROVINCES.map((prov) => (
                      <option key={prov} value={prov}>
                        {prov}
                      </option>
                    ))}
                  </select>
                </FormInput>
              </div>
            </div>
          </FormSection>

          <hr className="my-4" />

          {/* ── Bagian 4: Status ── */}
          <FormSection title="Status" icon="bi bi-toggles">
            <div className="row">
              <div className="col-md-6">
                <FormInput label="Status Aktif" error={errors.is_active}>
                  <select
                    className={`form-select ${errors.is_active ? 'is-invalid' : ''}`}
                    {...register('is_active')}
                  >
                    <option value="1">Aktif</option>
                    <option value="0">Tidak Aktif</option>
                  </select>
                </FormInput>
              </div>
            </div>
          </FormSection>

        </Modal.Body>

        <Modal.Footer>
          <button
            type="button"
            className="btn btn-secondary"
            onClick={onClose}
            disabled={loading}
          >
            Batal
          </button>

          <button
            type="submit"
            className="btn btn-primary"
            disabled={loading}
          >
            {loading ? (
              <>
                <span
                  className="spinner-border spinner-border-sm me-2"
                  role="status"
                  aria-hidden="true"
                />
                {isEdit ? 'Mengupdate...' : 'Menyimpan...'}
              </>
            ) : (
              isEdit ? 'Update' : 'Simpan'
            )}
          </button>
        </Modal.Footer>
      </form>
    </>
  );
}