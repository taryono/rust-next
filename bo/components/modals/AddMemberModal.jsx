// frontend/components/modals/AddMemberModal.jsx
'use client';

import { useState, useEffect, useCallback } from 'react';
import { Modal } from 'react-bootstrap';
import { useAuth } from '@/contexts/AuthContext';
import { api } from '@/lib/api';
import { alertSuccess, alertError } from '@/lib/alert';
import { useForm } from 'react-hook-form';
import SelectPro from '@/components/ui/SelectPro';
import ImageUpload from '@/components/ui/ImageUpload';
import FormSection from '@/components/ui/FormSection';

// Constants untuk validasi dan konfigurasi
const FORM_DEFAULTS = {
    name: 'Muhammad Syahdan Ksatria',
    email: 'muhammad.syahdan@gmail.com',
    dob: '2020-01-12', // ✅ Format YYYY-MM-DD
    pob: 'Brebes',
    password: 'password',
    password_confirm: 'password',
    role: [],
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
};



const PROVINCES = [
  'Jawa Barat', 'Jawa Tengah', 'Jawa Timur', 'DKI Jakarta',
  'Banten', 'Sumatera Utara', 'Sumatera Barat', 'Sumatera Selatan',
  'Bali', 'Kalimantan Timur'
];

const COUNTRIES = ['Indonesia', 'Malaysia', 'Singapore'];

const TIMEZONES = [
  { value: 'Asia/Jakarta', label: 'WIB (Jakarta)' },
  { value: 'Asia/Makassar', label: 'WITA (Makassar)' },
  { value: 'Asia/Jayapura', label: 'WIT (Jayapura)' }
];

// Custom hook untuk manajemen roles
const useRoles = () => {
  const [roles, setRoles] = useState([]);
  const [loadingRoles, setLoadingRoles] = useState(false);

  const fetchRoles = useCallback(async () => {
    setLoadingRoles(true);
    try {
      const response = await api.getRoles();
      if (response.data) {
        setRoles(response.data);
      }
    } catch (err) {
      console.error('Failed to fetch roles:', err);
      setRoles([]);
    } finally {
      setLoadingRoles(false);
    }
  }, []);

  return { roles, loadingRoles, fetchRoles };
};

// Fungsi untuk inspect FormData
const inspectFormData = (formData) => {
  console.log('=== INSPECT FORM DATA ===');
  for (let pair of formData.entries()) {
    console.log(`${pair[0]}:`, pair[1]);
  }
  console.log('=== END ===');
};

// Komponen untuk field yang sering digunakan
const FormInput = ({ label, error, children, required = false }) => (
  <div className="mb-3">
    <label className="form-label">
      {label} {required && <span className="text-danger">*</span>}
    </label>
    {children}
    {error && <div className="invalid-feedback d-block">{error.message}</div>}
  </div>
);

const GenderRadio = ({ register, error }) => (
  <FormInput label="Jenis Kelamin" error={error}>
    <div className="d-flex gap-3">
      {[
        { value: 'm', label: 'Laki-laki' },
        { value: 'f', label: 'Perempuan' }
      ].map((option) => (
        <div key={option.value} className="form-check">
          <input
            className="form-check-input"
            type="radio"
            id={`gender-${option.value}`}
            value={option.value} 
            {...register('gender')}
          />
          <label className="form-check-label" htmlFor={`gender-${option.value}`}>
            {option.label}
          </label>
        </div>
      ))}
    </div>
  </FormInput>
);

export default function AddMemberModal({ data, onClose, onSuccess }) {
  const { user } = useAuth();
  const [loading, setLoading] = useState(false);
  const { roles, loadingRoles, fetchRoles } = useRoles();
  const isEdit = !!data;
  console.log(user)
  // =====================================================
  // REACT HOOK FORM
  // =====================================================
  const {
    register,
    handleSubmit,
    control,
    setValue,
    watch,
    formState: { errors }
  } = useForm({
    defaultValues: FORM_DEFAULTS,
    mode: 'onBlur'
  });

  const password = watch('password');

  // =====================================================
  // EFFECTS
  // =====================================================
  useEffect(() => {
    fetchRoles();
  }, [fetchRoles]);

  // Setup data untuk mode edit
  useEffect(() => {
    if (data) {
      // Set semua field dari data
      Object.keys(FORM_DEFAULTS).forEach(key => {
        if (data[key] !== undefined) {
          setValue(key, data[key] ?? FORM_DEFAULTS[key]);
        }
      });

      // Normalisasi role
      if (data.role) {
        const normalizedRole = Array.isArray(data.role) 
          ? data.role 
          : [data.role].filter(Boolean);
        setValue('role', normalizedRole);
      }

      // Set image preview jika ada
      if (data.image) {
        setValue('image', data.image);
      }
    }
  }, [data, setValue]);

  // =====================================================
  // HANDLERS
  // =====================================================
  const handleImageUpload = useCallback((file) => {
    setValue('image', file, { shouldValidate: true });
  }, [setValue]);

  const onSubmit = async (formData) => {
    console.log('🔍 Original formData:', formData);
    console.log('🔍 Image value:', formData.image);
    console.log('🔍 Image type:', typeof formData.image);
    console.log('🔍 Is File?', formData.image instanceof File);
    setLoading(true);

    try {
        
   
      const formDataToSend = prepareFormData(formData);
      
      const response = isEdit
        ? await api.updateUser(data.id, formDataToSend)
        : await api.createMultipart(formDataToSend);

      alertSuccess(isEdit ? 'Member berhasil diperbarui' : 'Member berhasil ditambahkan');
      onSuccess?.(response);
      onClose();
    } catch (err) {
      const errorMessage = err.response?.data?.message || 
        `Gagal ${isEdit ? 'mengupdate' : 'menambahkan'} member`;
      alertError(errorMessage);
    } finally {
      setLoading(false);
    }
  };
 
const prepareFormData = (formData) => {
  console.log('🔍 Original formData:', formData);
  console.log('🔍 Image value:', formData.image);
  console.log('🔍 Image type:', typeof formData.image);
  console.log('🔍 Is File?', formData.image instanceof File);
  
  const formDataObj = new FormData();
  
  // Debug: tampilkan semua keys
  console.log('🔍 Form keys:', Object.keys(formData));
  
  // Loop melalui semua field
  Object.keys(formData).forEach(key => {
    const value = formData[key];
    
    if (value === null || value === undefined || value === '') {
      console.log(`⏭️ Skipping ${key}: empty`);
      return;
    }
    
    // Handle special cases
    if (key === 'role') {
      if (Array.isArray(value) && value.length > 0) {
        value.forEach(role => {
          console.log(`📝 Appending roles[]: ${role}`);
          formDataObj.append('roles[]', role);
        });
      }
    } else if (key === 'image') {
      if (value instanceof File) {
        console.log(`🖼️ Appending image:`, {
          name: value.name,
          size: value.size,
          type: value.type
        });
        formDataObj.append('image', value);
      } else {
        console.log(`❌ Image is not a File object:`, value);
      }
    } else {
      console.log(`📝 Appending ${key}: ${value}`);
      formDataObj.append(key, value);
    }
  });
  
  // Tambahkan foundation_id
  if (user?.foundation_id) {
    console.log(`📝 Appending foundation_id: ${user.foundation_id}`);
    formDataObj.append('foundation_id', user.foundation_id);
  }
  
  // Debug: tampilkan isi FormData
  console.log('🔍 FormData entries:');
  for (let [key, val] of formDataObj.entries()) {
    console.log(`  ${key}:`, val instanceof File ? `File(${val.name}, ${val.type})` : val);
  }
  
  return formDataObj;
};

  // =====================================================
  // RENDER
  // =====================================================
  return (
    <>
      <Modal.Header closeButton>
        <Modal.Title>
          {isEdit ? 'Edit Member' : 'Tambah Member Baru'}
        </Modal.Title>
      </Modal.Header>

      <form onSubmit={handleSubmit(onSubmit)}>
        <Modal.Body style={{ maxHeight: '70vh', overflowY: 'auto' }}>
          
          {/* Foto Profil */}
          <div className="mb-4 text-center">
            <ImageUpload
              onFileSelect={handleImageUpload}
              initialImage={data?.image}
              maxSize={2}
              label="Foto Profil"
            />
          </div>

          <hr className="my-4" />

          {/* Bagian 1: Informasi Dasar */}
          <FormSection
            title="Informasi Dasar"
            icon="bi bi-person-badge"
          >
            <div className="row">
              <div className="col-md-6">
                <FormInput label="Nama Lengkap" error={errors.name} required>
                  <input
                    className={`form-control ${errors.name ? 'is-invalid' : ''}`}
                    placeholder="Nama lengkap"
                    {...register('name', {
                      required: 'Nama wajib diisi',
                      minLength: { value: 3, message: 'Minimal 3 karakter' }
                    })}
                  />
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Email" error={errors.email} required>
                  <input
                    type="email"
                    className={`form-control ${errors.email ? 'is-invalid' : ''}`}
                    placeholder="email@contoh.com"
                    {...register('email', {
                      required: 'Email wajib diisi',
                      pattern: {
                        value: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
                        message: 'Format email tidak valid'
                      }
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <div className="row">
              <div className="col-md-6">
                <FormInput label="Tanggal Lahir" error={errors.dob}>
                  <input
                    type="date"
                    className={`form-control ${errors.dob ? 'is-invalid' : ''}`}
                    {...register('dob')}
                  />
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Tempat Lahir" error={errors.pob}>
                  <input
                    className={`form-control ${errors.pob ? 'is-invalid' : ''}`}
                    placeholder="Tempat lahir"
                    {...register('pob')}
                  />
                </FormInput>
              </div>
            </div>

            <div className="row">
              <div className="col-md-6">
                <GenderRadio register={register} error={errors.gender} />
              </div>

              <div className="col-md-6">
                <FormInput label="Nomor Handphone" error={errors.phone}>
                  <input
                    className={`form-control ${errors.phone ? 'is-invalid' : ''}`}
                    placeholder="08xxxxxxxxxx"
                    {...register('phone', {
                      pattern: {
                        value: /^[0-9]{9,13}$/,
                        message: 'Nomor handphone tidak valid'
                      }
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <FormInput label="Bio" error={errors.bio}>
              <textarea
                className={`form-control ${errors.bio ? 'is-invalid' : ''}`}
                placeholder="Deskripsi singkat tentang member"
                rows="3"
                {...register('bio', {
                  maxLength: {
                    value: 500,
                    message: 'Maksimal 500 karakter'
                  }
                })}
              />
            </FormInput>
          </FormSection>

          <hr className="my-4" />

          {/* Bagian 2: Pengaturan Akun */}
          <FormSection
            title="Pengaturan Akun"
            icon="bi bi-shield-lock"
          >
            <div className="row">
              <div className="col-md-6">
                <FormInput 
                  label={`Password ${!isEdit ? '*' : ''}`} 
                  error={errors.password}
                  required={!isEdit}
                >
                  <input
                    type="password"
                    className={`form-control ${errors.password ? 'is-invalid' : ''}`}
                    placeholder={isEdit ? "Kosongkan jika tidak ingin ganti" : "Password"}
                    {...register('password', {
                      required: isEdit ? false : 'Password wajib diisi',
                      minLength: {
                        value: 6,
                        message: 'Password minimal 6 karakter'
                      }
                    })}
                  />
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput 
                  label={`Konfirmasi Password ${!isEdit ? '*' : ''}`} 
                  error={errors.password_confirm}
                  required={!isEdit}
                >
                  <input
                    type="password"
                    className={`form-control ${errors.password_confirm ? 'is-invalid' : ''}`}
                    placeholder="Ulangi password"
                    {...register('password_confirm', {
                      required: isEdit ? false : 'Konfirmasi password wajib diisi',
                      validate: value => {
                        if (isEdit && !password) return true;
                        return value === password || "Konfirmasi password tidak sama";
                      }
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <div className="row">
                <div className="col-md-12">
                  <FormInput label="Role" error={errors.role}>
                    <span className="text-danger"> *</span>
                    <SelectPro
                      name="role"
                      label="Role"
                      control={control}
                      options={roles}
                      textKey="name"
                      valueKey="id"
                      isLoading={loadingRoles}
                      isMulti
                      rules={{
                        validate: value =>
                          Array.isArray(value) && value.length > 0
                            ? true
                            : "Minimal pilih 1 role"
                      }}
                    />
                  </FormInput>
                </div>
              </div>
              <div className="row"> 
                <div className="col-md-12">
                  <FormInput label="Status" error={errors.status}>
                    <select
                      className={`form-select ${errors.status ? 'is-invalid' : ''}`}
                      {...register('status')}
                    >
                      <option value="0">Inactive</option>
                      <option value="1">Active</option>
                      <option value="2">Pending</option>
                    </select>
                  </FormInput>
                </div>
            </div>
          </FormSection>

          <hr className="my-4" />

          {/* Bagian 3: Informasi Alamat */}
          <FormSection
            title="Informasi Alamat"
            icon="bi bi-geo-alt"
          >
            <FormInput label="Alamat Lengkap" error={errors.address}>
              <textarea
                rows="2"
                className={`form-control ${errors.address ? 'is-invalid' : ''}`}
                placeholder="Jalan, RT/RW, Kelurahan/Desa"
                {...register('address')}
              />
            </FormInput>

            <div className="row">
              <div className="col-md-6">
                <FormInput label="Kota/Kabupaten" error={errors.city}>
                  <input
                    className={`form-control ${errors.city ? 'is-invalid' : ''}`}
                    placeholder="Contoh: Brebes"
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
                    <option value="">Pilih Provinsi</option>
                    {PROVINCES.map(prov => (
                      <option key={prov} value={prov}>{prov}</option>
                    ))}
                  </select>
                </FormInput>
              </div>
            </div>

            <div className="row">
              <div className="col-md-6">
                <FormInput label="Negara" error={errors.country}>
                  <select
                    className={`form-select ${errors.country ? 'is-invalid' : ''}`}
                    {...register('country')}
                  >
                    {COUNTRIES.map(country => (
                      <option key={country} value={country}>{country}</option>
                    ))}
                  </select>
                </FormInput>
              </div>

              <div className="col-md-6">
                <FormInput label="Kode Pos" error={errors.postal_code}>
                  <input
                    className={`form-control ${errors.postal_code ? 'is-invalid' : ''}`}
                    placeholder="Contoh: 52262"
                    {...register('postal_code', {
                      pattern: {
                        value: /^[0-9]{5}$/,
                        message: 'Kode pos harus 5 digit angka'
                      }
                    })}
                  />
                </FormInput>
              </div>
            </div>

            <div className="row">
              <div className="col-md-4">
                <FormInput label="Latitude" error={errors.latitude}>
                  <input
                    className={`form-control ${errors.latitude ? 'is-invalid' : ''}`}
                    placeholder="Contoh: -6.9175"
                    {...register('latitude')}
                  />
                </FormInput>
              </div>

              <div className="col-md-4">
                <FormInput label="Longitude" error={errors.longitude}>
                  <input
                    className={`form-control ${errors.longitude ? 'is-invalid' : ''}`}
                    placeholder="Contoh: 107.6191"
                    {...register('longitude')}
                  />
                </FormInput>
              </div>

              <div className="col-md-4">
                <FormInput label="Timezone" error={errors.timezone}>
                  <select
                    className={`form-select ${errors.timezone ? 'is-invalid' : ''}`}
                    {...register('timezone')}
                  >
                    {TIMEZONES.map(tz => (
                      <option key={tz.value} value={tz.value}>{tz.label}</option>
                    ))}
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
                <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
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


// Mode RHF
/* <SelectPro
  name="categories"
  control={control}
  options={categoryOptions}
  label="Kategori"
  isMandatory
  multiple
/>

// Mode biasa
<SelectPro
  name="tags"
  options={tagOptions}
  value={selectedTags}
  onChange={(name, val) => setSelectedTags(val)}
  label="Tags"
  multiple={false}
/> */