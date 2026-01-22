// frontend/store/uiStore.js
import { create } from 'zustand';

const useUIStore = create((set, get) => ({
  // State
  sidebarOpen: false,
  sidebarCollapsed: false,
  expandedMenus: [], // Array of expanded menu keys

  // Actions
  toggleSidebar: () => {
    const isMobile = typeof window !== 'undefined' && window.innerWidth < 768;
    
    if (isMobile) {
      set((state) => ({ sidebarOpen: !state.sidebarOpen }));
    } else {
      set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed }));
    }
  },

  openSidebar: () => set({ sidebarOpen: true }),
  closeSidebar: () => set({ sidebarOpen: false }),
  
  toggleCollapse: () => 
    set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
  
  expandSidebar: () => set({ sidebarCollapsed: false }),
  collapseSidebar: () => set({ sidebarCollapsed: true }),

  // Tree Menu Actions
  toggleMenu: (menuKey) => {
    console.log('Toggle menu called:', menuKey); // DEBUG
    set((state) => {
      const isCurrentlyExpanded = state.expandedMenus.includes(menuKey);
      console.log('Currently expanded:', isCurrentlyExpanded); // DEBUG
      console.log('Expanded menus before:', state.expandedMenus); // DEBUG
      
      const newExpandedMenus = isCurrentlyExpanded
        ? state.expandedMenus.filter(key => key !== menuKey)
        : [...state.expandedMenus, menuKey];
      
      console.log('Expanded menus after:', newExpandedMenus); // DEBUG
      
      return { expandedMenus: newExpandedMenus };
    });
  },

  expandMenu: (menuKey) =>
    set((state) => ({
      expandedMenus: state.expandedMenus.includes(menuKey)
        ? state.expandedMenus
        : [...state.expandedMenus, menuKey]
    })),

  collapseMenu: (menuKey) =>
    set((state) => ({
      expandedMenus: state.expandedMenus.filter(key => key !== menuKey)
    })),

  collapseAllMenus: () => set({ expandedMenus: [] }),

  reset: () => 
    set({ 
      sidebarOpen: false, 
      sidebarCollapsed: false,
      expandedMenus: []
    }),
}));

export default useUIStore;