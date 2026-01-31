import { create } from 'zustand';

const useModalStore = create((set) => ({
    show: false,
    type: null,
    data: null,
    onSuccess: null, // ✅ Tambahkan

    openModal: (type, data = null, onSuccess = null) =>
        set({
            show: true,
            type,
            data,
            onSuccess
        }),

    closeModal: () =>
        set({
            show: false,
            type: null,
            data: null,
            onSuccess:null,
        }),
}));

export default useModalStore;
