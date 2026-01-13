import { create } from 'zustand';

export default create((set) => ({
  toast: null,
  showToast: (toast) => set({ toast }),
  clearToast: () => set({ toast: null }),
}));
