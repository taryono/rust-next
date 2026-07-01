// dashboard/page.jsx
'use client';
import AuthLayout from '@/components/layout/AuthLayout';    
import { useState, useEffect } from 'react';

export default function DashboardPage() {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setTimeout(() => {
      setLoading(false);
    }, 1000);
  }, []);

  if (loading) {
      return (
        <AuthLayout>
          <div className="page">
            <div className="page-wrapper">
              <div className="container-xl d-flex flex-column justify-content-center" style={{minHeight: '100vh'}}>
                <div className="text-center text-white">
                  <div className="spinner-border text-primary font-weight-bold font-size-lg" role="status"></div>
                    <div>
                      <span className="spinner-border spinner-border-sm me-2" role="status" aria-hidden="true"></span>
                    </div>
                      Loading ... 
                </div>
              </div>
            </div>
          </div>
        </AuthLayout>
      );
    } 

  return (
    <AuthLayout>
      <div className="page">
          <div className="page-wrapper"> 
            {/* Page header */}
            <div className="page-header d-print-none">
              <div className="container-xl">
                <div className="row g-2 align-items-center">
                  <div className="col">
                    <div className="page-pretitle">Dashboad</div> 
                  </div> 
                </div>
              </div>
            </div>
            {/* Body */}
            <div className="page-body">
              <div className="container-xl"> 
                {/* Welcome Section */}
                <div className="row mb-4">
                    <div className="col-12">
                        <div className="card dashboard-card border-0 fade-in">
                            <div className="card-body p-4">
                                <h2 className="card-title mb-3">
                                    <i className="bi bi-house-heart me-2 text-primary"></i>
                                    Welcome, 
                                </h2>
                                <p className="text-muted mb-0">
                                    Manage your family tree and explore your heritage.
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
              </div>
            </div>
          </div>
      </div>   
    </AuthLayout>
  );
}
