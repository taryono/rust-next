'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import useAuthStore from '@/store/authStore';

export default function Home() {
  const router = useRouter();
  const { initialize, isAuthenticated } = useAuthStore();

  useEffect(() => {
    initialize();

    // Redirect based on auth status
    if (isAuthenticated) {
      router.push('/dashboard');
    } else {
      router.push('/login');
    }
  }, [isAuthenticated, router, initialize]);

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