import { create } from 'zustand';

const useModalStore = create((set) => ({
    show: false,
    type: null,
    data: null,

    openModal: (type, data = null) =>
        set({
            show: true,
            type,
            data,
        }),

    closeModal: () =>
        set({
            show: false,
            type: null,
            data: null,
        }),
}));

export default useModalStore;
