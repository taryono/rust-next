// frontend/components/ui/FormSection.jsx
'use client';

import { useState } from 'react';

const FormSection = ({ 
  title, 
  icon, 
  children, 
  collapsible = false,
  defaultOpen = true,
  className = "",
  headerClassName = "",
  bodyClassName = ""
}) => {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  return (
    <div className={`form-section mb-4 ${className}`}>
      {/* Header Section */}
      <div 
        className={`
          form-section-header 
          d-flex align-items-center justify-content-between 
          mb-3 pb-2 border-bottom
          ${headerClassName}
        `}
        style={{ cursor: collapsible ? 'pointer' : 'default' }}
        onClick={() => collapsible && setIsOpen(!isOpen)}
      >
        <div className="d-flex align-items-center">
          {icon && (
            <i 
              className={`${icon} me-2 text-primary`} 
              style={{ fontSize: '1.25rem' }}
            />
          )}
          <h5 className="mb-0 text-primary fw-semibold">
            {title}
          </h5>
        </div>

        {collapsible && (
          <i 
            className={`bi ${isOpen ? 'bi-chevron-up' : 'bi-chevron-down'} text-muted`}
            style={{ transition: 'transform 0.2s ease' }}
          />
        )}
      </div>

      {/* Body Section */}
      <div 
        className={`form-section-body ${bodyClassName}`}
        style={{
          display: isOpen || !collapsible ? 'block' : 'none',
          animation: isOpen ? 'slideDown 0.3s ease' : 'none'
        }}
      >
        {children}
      </div>

      {/* Inline styles untuk animasi */}
      <style jsx>{`
        @keyframes slideDown {
          from {
            opacity: 0;
            transform: translateY(-10px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }
        
        .form-section {
          transition: all 0.3s ease;
        }
        
        .form-section-header:hover {
          ${collapsible ? 'opacity: 0.8;' : ''}
        }
      `}</style>
    </div>
  );
};

// Variasi tambahan untuk styling berbeda
export const FormSectionCard = ({ 
  title, 
  icon, 
  children, 
  variant = 'default',
  ...props 
}) => {
  const variants = {
    default: 'border rounded p-4 bg-white',
    primary: 'border border-primary rounded p-4 bg-primary-subtle',
    secondary: 'border rounded p-4 bg-light',
    ghost: 'p-0'
  };

  return (
    <FormSection
      title={title}
      icon={icon}
      className={variants[variant] || variants.default}
      {...props}
    >
      {children}
    </FormSection>
  );
};

export const FormSectionSimple = ({ 
  title, 
  children, 
  ...props 
}) => (
  <FormSection
    title={title}
    className="mb-3"
    headerClassName="mb-2"
    bodyClassName="ps-3"
    {...props}
  >
    {children}
  </FormSection>
);

export default FormSection;

// // Basic
// <FormSection title="Informasi Dasar" icon="bi bi-person-badge">
//   {/* Form fields di sini */}
// </FormSection>

// // Collapsible
// <FormSection 
//   title="Alamat" 
//   icon="bi bi-geo-alt"
//   collapsible
//   defaultOpen={false}
// >
//   {/* Form fields di sini */}
// </FormSection>

// // Dengan styling card
// <FormSectionCard 
//   title="Pengaturan Akun" 
//   icon="bi bi-shield-lock"
//   variant="primary"
// >
//   {/* Form fields di sini */}
// </FormSectionCard>