import Swal from 'sweetalert2';

const Toast = Swal.mixin({
    toast: true,
    position: 'top-end',
    showConfirmButton: false,
    timer: 3000,
    timerProgressBar: true,
    didOpen: (toast) => {
        toast.onmouseenter = Swal.stopTimer;
        toast.onmouseleave = Swal.resumeTimer;
    },
    customClass: {
        popup: 'colored-toast',
    },
});

export const toastSuccess = (message) =>
    Toast.fire({
        icon: 'success',
        title: message,
    });

export const toastError = (message) =>
    Toast.fire({
        icon: 'error',
        title: message,
        iconColor: '#ffffff',
        color: 'white',
    });

export const toastInfo = (message) =>
    Toast.fire({
        icon: 'info',
        title: message,
    });

export const toastWarning = (message) =>
    Toast.fire({
        icon: 'warning',
        title: message,
    }); 