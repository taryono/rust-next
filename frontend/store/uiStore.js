import { create } from 'zustand';

const useUIStore = create((set) => ({
  sidebarOpen: false,
  sidebarCollapsed: false,

  toggleSidebarMobile: () =>
    set((s) => ({ sidebarOpen: !s.sidebarOpen })),

  toggleSidebarDesktop: () =>
    set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),

  closeSidebar: () => set({ sidebarOpen: false }),
}));


export default useUIStore;
