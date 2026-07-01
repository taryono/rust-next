// frontend/components/ui/ImageUpload.jsx
'use client';

import { useState, useRef } from 'react';
import { alertError } from '@/lib/alert';
import "../../public/styles/components.css";
const ImageUpload = ({ 
  onFileSelect, 
  initialImage = null,
  maxSize = 2, // dalam MB
  label = "Foto",
  aspectRatio = "1/1",
  className = "",
  previewClassName = "",
  disabled = false
}) => {
  const [preview, setPreview] = useState(initialImage);
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef(null);

  const validateFile = (file) => {
    // Validasi tipe file
    if (!file.type.startsWith('image/')) {
      alertError('File harus berupa gambar');
      return false;
    }

    // Validasi ukuran file
    if (file.size > maxSize * 1024 * 1024) {
      alertError(`Ukuran file maksimal ${maxSize}MB`);
      return false;
    }

    return true;
  };

  const handleFileChange = (file) => {
    if (!file || !validateFile(file)) return;

    // Buat preview
    const reader = new FileReader();
    reader.onloadend = () => {
      setPreview(reader.result);
    };
    reader.readAsDataURL(file);

    // Kirim file ke parent component
    onFileSelect(file);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    setIsDragging(false);
    
    const file = e.dataTransfer.files[0];
    handleFileChange(file);
  };

  const handleDragOver = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleClick = () => {
    if (!disabled && fileInputRef.current) {
      fileInputRef.current.click();
    }
  };

  return (
    <div className={`image-upload-wrapper ${className}`}>
      <label className="form-label fw-bold d-block mb-3">
        {label}
      </label>

      <div className="d-flex flex-column align-items-center">
        {/* Preview Area */}
        <div
          className={`
            image-preview-container 
            ${previewClassName}
            ${isDragging ? 'dragging' : ''}
            ${disabled ? 'disabled' : ''}
          `}
          style={{
            width: 140,
            height: 140,
            borderRadius: '50%',
            overflow: 'hidden',
            cursor: disabled ? 'not-allowed' : 'pointer',
            position: 'relative',
            aspectRatio: aspectRatio,
            border: isDragging 
              ? '3px dashed var(--bs-primary)' 
              : '2px solid var(--bs-border-color)',
            backgroundColor: isDragging 
              ? 'var(--bs-primary-bg-subtle)' 
              : 'var(--bs-secondary-bg)',
            transition: 'all 0.2s ease'
          }}
          onClick={handleClick}
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          title={disabled ? '' : 'Klik atau drag & drop gambar'}
        >
          {preview ? (
            <img
              src={preview}
              alt="Preview"
              style={{
                width: '100%',
                height: '100%',
                objectFit: 'cover'
              }}
            />
          ) : (
            <div className="d-flex flex-column align-items-center justify-content-center h-100">
              <i 
                className="bi bi-person-fill" 
                style={{ 
                  fontSize: '3rem',
                  color: 'var(--bs-secondary)'
                }}
              />
              <small className="mt-2 text-muted">
                Upload Foto
              </small>
            </div>
          )}

          {/* Overlay untuk hover effect */}
          {!disabled && (
            <div 
              className="image-upload-overlay"
              style={{
                position: 'absolute',
                top: 0,
                left: 0,
                right: 0,
                bottom: 0,
                backgroundColor: 'rgba(0,0,0,0.5)',
                opacity: 0,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                color: 'white',
                transition: 'opacity 0.2s ease'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.opacity = '1';
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.opacity = '0';
              }}
            >
              <i className="bi bi-camera-fill fs-4"></i>
            </div>
          )}

          {/* Remove button jika ada preview */}
          {preview && !disabled && (
            <button
              type="button"
              className="btn btn-danger btn-sm rounded-circle"
              style={{
                position: 'absolute',
                top: 8,
                right: 8,
                width: 28,
                height: 28,
                padding: 0,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center'
              }}
              onClick={(e) => {
                e.stopPropagation();
                setPreview(null);
                onFileSelect(null);
                if (fileInputRef.current) {
                  fileInputRef.current.value = '';
                }
              }}
            >
              <i className="bi bi-x"></i>
            </button>
          )}
        </div>

        {/* File Input (hidden) */}
        <input
          ref={fileInputRef}
          type="file"
          accept="image/*"
          onChange={(e) => handleFileChange(e.target.files[0])}
          style={{ display: 'none' }}
          disabled={disabled}
        />

        {/* Informasi upload */}
        <div className="mt-3 text-center">
          <small className="text-muted">
            Format: JPG, PNG, GIF (Maks. {maxSize}MB)
          </small>
          
          {isDragging && (
            <div className="mt-2 text-primary small">
              <i className="bi bi-cloud-arrow-up me-1"></i>
              Lepaskan gambar di sini...
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default ImageUpload;