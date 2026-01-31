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
 
export function printErrors(error) {
    console.error('API Error:', error);
    
    let icon = 'error';
    let title = 'Error';
    let text = 'An error occurred';
    let html = null;
    
    // Handle Axios/Fetch error response
    const response = error.response || error;
    const status = response?.status || 0;
    const data = response?.data || {};
    
    if (status === 404) {
        title = 'Not Found';
        text = 'Resource not found (404)';
    } else if (status === 500) {
        title = 'Server Error';
        text = 'Internal server error occurred (500)';
    } else if (status === 422) {
        // Validation Errors
        title = 'Validation Error';
        icon = 'warning';
        
        const errors = data.errors || {};
        if (Object.keys(errors).length > 0) {
            let errorList = '<ul class="text-start">';
            Object.entries(errors).forEach(([field, messages]) => {
                if (Array.isArray(messages)) {
                    messages.forEach(message => {
                        errorList += `<li>${message}</li>`;
                    });
                } else {
                    errorList += `<li>${messages}</li>`;
                }
            });
            errorList += '</ul>';
            html = errorList;
            text = null;
        } else {
            text = data.message || 'Validation failed';
        }
    } else if (status === 401) {
        title = 'Unauthorized';
        text = 'Your session has expired. Please login again.';
        icon = 'warning';
    } else if (status === 403) {
        title = 'Forbidden';
        text = 'You do not have permission to perform this action.';
        icon = 'warning';
    } else if (status === 0 || error.message === 'Network Error') {
        title = 'Network Error';
        text = 'Could not connect to server. Please check your internet connection.';
        icon = 'error';
    } else {
        text = data.message || error.message || 'Unknown error';
    }
    
    return Swal.fire({
        icon: icon,
        title: title,
        text: text,
        html: html,
        confirmButtonText: 'OK',
        allowOutsideClick: false
    }).then((result) => {
        if (result.isConfirmed) {
            if (status === 401) {
                window.location.href = '/login';
            }
        }
    });
}

export function showSuccess(message, title = 'Success') {
    return Swal.fire({
        icon: 'success',
        title: title,
        text: message,
        timer: 2000,
        showConfirmButton: false
    });
}

export function showError(message, title = 'Error') {
    return Swal.fire({
        icon: 'error',
        title: title,
        text: message,
        confirmButtonText: 'OK'
    });
}

export function showWarning(message, title = 'Warning') {
    return Swal.fire({
        icon: 'warning',
        title: title,
        text: message,
        confirmButtonText: 'OK'
    });
}

export function showConfirm(message, title = 'Are you sure?') {
    return Swal.fire({
        icon: 'question',
        title: title,
        text: message,
        showCancelButton: true,
        confirmButtonText: 'Yes',
        cancelButtonText: 'Cancel',
        confirmButtonColor: '#d33',
        cancelButtonColor: '#3085d6',
    });
}

export function showToast(message, icon = 'success') {
    const Toast = Swal.mixin({
        toast: true,
        position: 'top-end',
        showConfirmButton: false,
        timer: 3000,
        timerProgressBar: true,
        didOpen: (toast) => {
            toast.addEventListener('mouseenter', Swal.stopTimer);
            toast.addEventListener('mouseleave', Swal.resumeTimer);
        }
    });
    
    return Toast.fire({
        icon: icon,
        title: message
    });
}