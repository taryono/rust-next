// store/teacherStore.js
import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export const useTeacherStore = create(
  devtools((set, get) => ({
    teachers: [],
    selectedTeacher: null,
    loading: false,
    error: null,

    fetchTeachers: async (filters = {}) => {
      set({ loading: true, error: null });
      try {
        const params = new URLSearchParams(filters);
        const response = await fetch(`/api/teachers?${params.toString()}`);
        if (!response.ok) throw new Error('Failed to fetch teachers');

        const data = await response.json();
        set({ teachers: data, loading: false });
      } catch (error) {
        set({
          error: error?.message || 'Unexpected error',
          loading: false,
        });
      }
    },

    fetchTeacherDetail: async (id) => {
      set({ loading: true, error: null });
      try {
        const response = await fetch(`/api/teachers/${id}`);
        if (!response.ok) throw new Error('Failed to fetch teacher');

        const data = await response.json();
        set({ selectedTeacher: data, loading: false });
      } catch (error) {
        set({
          error: error?.message || 'Unexpected error',
          loading: false,
        });
      }
    },

    createTeacher: async (data) => {
      set({ loading: true, error: null });
      try {
        const response = await fetch('/api/teachers', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(data),
        });

        if (!response.ok) throw new Error('Failed to create teacher');

        await get().fetchTeachers();
        set({ loading: false });
      } catch (error) {
        set({
          error: error?.message || 'Unexpected error',
          loading: false,
        });
      }
    },

    updateTeacher: async (id, data) => {
      set({ loading: true, error: null });
      try {
        const response = await fetch(`/api/teachers/${id}`, {
          method: 'PUT',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(data),
        });

        if (!response.ok) throw new Error('Failed to update teacher');

        await get().fetchTeachers();
        set({ loading: false });
      } catch (error) {
        set({
          error: error?.message || 'Unexpected error',
          loading: false,
        });
      }
    },

    deleteTeacher: async (id) => {
      set({ loading: true, error: null });
      try {
        const response = await fetch(`/api/teachers/${id}`, {
          method: 'DELETE',
        });

        if (!response.ok) throw new Error('Failed to delete teacher');

        await get().fetchTeachers();
        set({ loading: false });
      } catch (error) {
        set({
          error: error?.message || 'Unexpected error',
          loading: false,
        });
      }
    },
  }))
);
