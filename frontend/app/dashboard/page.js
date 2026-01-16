// dashboard/page.jsx
import AuthLayout from '@/components/layout/AuthLayout';

export default function DashboardPage() {
  return (
    <AuthLayout>
     <div className="dashboard-container">
      <div className="container py-5">
        <h1>Dashboard</h1>
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
    </AuthLayout>
  );
}
