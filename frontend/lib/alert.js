import Swal from 'sweetalert2';

const baseConfig = {
    buttonsStyling: false,
    customClass: {
        confirmButton: 'btn btn-primary me-2',
        cancelButton: 'btn btn-secondary',
    },
};

export const alertSuccess = (title, text) =>
    Swal.fire({
        ...baseConfig,
        icon: 'success',
        title,
        text,
        timer: 2000,
        showConfirmButton: false,
    });

export const alertError = (title, text) =>
    Swal.fire({
        ...baseConfig,
        icon: 'error',
        title,
        text,
    });

export const alertConfirm = (title, text) =>
    Swal.fire({
        ...baseConfig,
        icon: 'warning',
        title,
        text,
        showCancelButton: true,
        confirmButtonText: 'Ya',
        cancelButtonText: 'Batal',
    });
