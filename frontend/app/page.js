'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import useAuthStore from '@/store/authStore';

export default function Home() {
  const router = useRouter();
  const { initialize, isAuthenticated } = useAuthStore();

  // Initialize auth state once
  useEffect(() => {
    initialize();
  }, []); // Empty dependency = run once on mount

  // Handle redirect based on auth status
  useEffect(() => {
    const redirect = isAuthenticated ? '/dashboard' : '/login';
    router.replace(redirect); // Use replace, not push
  }, [isAuthenticated, router]);

  return (
    <div className="loading-container">
      <div className="text-center">
        <div className="spinner-border spinner-border-custom text-white" role="status">
          <span className="visually-hidden">Loading...</span>
        </div>
        <p className="mt-3 text-white fw-semibold fs-5">Loading Silsilah App...</p>
      </div>
    </div>
  );
}