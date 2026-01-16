import { create } from 'zustand';

const useUIStore = create((set) => ({
  // State
  sidebarOpen: false,        // Mobile: apakah sidebar visible
  sidebarCollapsed: false,   // Desktop: apakah sidebar collapsed

  // Actions
  toggleSidebar: () => 
    set((state) => ({ sidebarOpen: !state.sidebarOpen })),

  openSidebar: () => 
    set({ sidebarOpen: true }),

  closeSidebar: () => 
    set({ sidebarOpen: false }),

  toggleCollapse: () => 
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),

  // Reset (useful for logout)
  reset: () => 
    set({ sidebarOpen: false, sidebarCollapsed: false }),
}));

export default useUIStore;