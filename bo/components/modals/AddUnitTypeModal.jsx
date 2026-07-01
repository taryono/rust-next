'use client';

import { useState, useEffect } from 'react';
import { Modal } from 'react-bootstrap';
import { useForm } from 'react-hook-form';
import { api } from '@/lib/api';
import { alertSuccess, alertError } from '@/lib/alert';
import FormSection from '@/components/ui/FormSection';

const FORM_DEFAULTS = {
  name: '',
  code: '',
  description: '',
  is_active: '1',
};

const FormInput = ({ label, error, children, required = false }) => (
  <div className="mb-3">
    <label className="form-label fw-semibold">
      {label} {required && <span className="text-danger">*</span>}
    </label>
    {children}
    {error && <div className="invalid-feedback d-block">{error.message}</div>}
  </div>
);

export default function AddUnitTypeModal({ data, onClose, onSuccess }) {
  const [loading, setLoading] = useState(false);
  const isEdit = !!data;

  const {
    register,
    handleSubmit,
    setValue,
    formState: { errors },
  } = useForm({ defaultValues: FORM_DEFAULTS, mode: 'onBlur' });

  // Populate form saat edit
  useEffect(() => {
    if (data) {
      Object.keys(FORM_DEFAULTS).forEach((key) => {
        if (data[key] !== undefined) {
          setValue(key, data[key] ?? FORM_DEFAULTS[key]);
        }
      });
    }
  }, [data, setValue]);

  const preparePayload = (formData) => {
    const payload = { ...formData };

    // Field opsional → null jika kosong
    ['code', 'description'].forEach((key) => {
      if (payload[key] === '' || payload[key] === undefined) {
        payload[key] = null;
      }
    });

    payload.is_active = Number(payload.is_active);

    return payload;
  };

  const onSubmit = async (formData) => {
    setLoading(true);
    try {
      const payload = preparePayload(formData);
      const response = isEdit
        ? await api.updateUnitType(data.id, payload)
        : await api.createUnitType(payload);

      alertSuccess(isEdit ? 'Unit type berhasil diperbarui' : 'Unit type berhasil ditambahkan');
      onSuccess?.(response);
      onClose();
    } catch (err) {
      const errorMessage =
        err.response?.data?.message ||
        `Gagal ${isEdit ? 'mengupdate' : 'menambahkan'} unit type`; // ✅ fix teks
      alertError(errorMessage);
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <Modal.Header closeButton>
        <Modal.Title>
          <i className="ti ti-category me-2"></i>
          {isEdit ? 'Edit Unit Type' : 'Tambah Unit Type Baru'}
        </Modal.Title>
      </Modal.Header>

      <form onSubmit={handleSubmit(onSubmit)}>
        <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}>

          <FormSection title="Informasi Utama" icon="bi bi-tag">
            <div className="row">
              <div className="col-md-8">
                <FormInput label="Nama Unit Type" error={errors.name} required>
                  <input
                    className={`form-control ${errors.name ? 'is-invalid' : ''}`}
                    placeholder="Contoh: Unit Pendidikan"
                    {...register('name', {
                      required: 'Nama unit type wajib diisi', // ✅ fix teks
                      minLength: { value: 3, message: 'Minimal 3 karakter' },
                    })}
                  />
                </FormInput>
              </div>

              <div className="col-md-4">
                <FormInput label="Kode" error={errors.code}>
                  <input
                    className={`form-control ${errors.code ? 'is-invalid' : ''}`}
                    placeholder="Contoh: UT-001"
                    {...register('code', {
                      maxLength: { value: 50, message: 'Maksimal 50 karakter' },
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <FormInput label="Deskripsi" error={errors.description}>
              <textarea
                className={`form-control ${errors.description ? 'is-invalid' : ''}`}
                placeholder="Deskripsi singkat tentang unit type"
                rows="3"
                {...register('description', {
                  maxLength: { value: 1000, message: 'Maksimal 1000 karakter' },
                })}
              />
            </FormInput>
          </FormSection>

          <hr className="my-4" />

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
          <button type="button" className="btn btn-secondary" onClick={onClose} disabled={loading}>
            Batal
          </button>
          <button type="submit" className="btn btn-primary" disabled={loading}>
            {loading ? (
              <>
                <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true" />
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