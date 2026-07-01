// frontend/app/dashboard/departments/page.js
'use client';

import { useState,useEffect } from 'react';
import AuthLayout from '@/components/layout/AuthLayout';
import { api } from '@/lib/api';
import { alertError, alertSuccess } from '@/lib/alert';
import { showConfirm } from '@/lib/toast';
import { usePagination } from '@/hooks/usePagination';
import Pagination from '@/components/common/Pagination'; 
import useModalStore from '@/store/modalStore';
import CardHeader from '@/components/ui/CardHeader';
import Loader from '@/components/ui/Loader';
import AddButton from '@/components/ui/AddButton';
import TableFilters from '@/components/ui/TableFilters';
import { setUnauthorizedHandler } from '@/lib/api';
import { useAuth } from '@/contexts/AuthContext';  


export default function Departments() {
  const { openModal } = useModalStore(); 
  const [viewMode, setViewMode] = useState('grid');
  const { handleUnauthorized,user } = useAuth(); 
 
  // Setup unauthorized handler on mount
  useEffect(() => {
    setUnauthorizedHandler(handleUnauthorized);
    
    return () => {
      setUnauthorizedHandler(null);
    };
  }, [handleUnauthorized]);
  // Gunakan hook pagination (ini sudah handle semuanya)
  const {
    data: departments,
    loading,
    error,
    pagination,
    filters,
    goToPage,
    changePerPage,
    updateFilters,
    refresh, // tambahkan ini untuk refresh data
  } = usePagination(api.getDepartments);

  // Helper functions
  const getDepartmentBadgeColor = (department) => {
    const colors = {
      'Admin': 'bg-red',
      'Editor': 'bg-blue',
      'Viewer': 'bg-green',
      'default': 'bg-gray'
    };
    return colors[department] || colors.default;
  };

  const getAvatarColor = (index) => {
    const colors = [
      'bg-blue-lt', 'bg-azure-lt', 'bg-indigo-lt', 'bg-purple-lt', 
      'bg-pink-lt', 'bg-red-lt', 'bg-orange-lt', 'bg-yellow-lt'
    ];
    return colors[index % colors.length];
  };

  const getInitials = (name) => {
    return name?.split(' ').map(n => n[0]).join('').toUpperCase() || '??';
  };

  // Handle edit department
  const handleEditDepartment = (departmentId) => {
    const department = departments.find(u => u.id === departmentId);
    openModal('add-member', department, refresh);
  };

  // Handle delete department
  const handleDeleteDepartment = async (departmentId) => {
    const result = await showConfirm(
      'This department will be permanently deleted.',
      'Delete Department?'
    );
    
    if (result.isConfirmed) {
      try {
        await api.deleteDepartment(departmentId); // pastikan ada di api.js
        alertSuccess('Department deleted successfully!');
        refresh(); // refresh data setelah delete
      } catch (error) {
        alertError(error.response?.data?.message || 'Failed to delete department');
      }
    }
  }; 

  // Loading state
  if (loading && departments.length === 0) {
    return (
      <AuthLayout>
        <Loader title="Loading Departments..." /> 
      </AuthLayout>
    );
  }

  // Error state
  if (error) {
    return (
      <AuthLayout>
        <div className="container-xl d-flex flex-column justify-content-center">
          <div className="empty">
            <div className="empty-icon">
              <i className="bi bi-exclamation-triangle"></i>
            </div>
            <p className="empty-title">Failed to load departments</p>
            <p className="empty-subtitle text-secondary">{error}</p>
            <div className="empty-action">
              <button className="btn btn-primary" onClick={refresh}>
                <i className="bi bi-arrow-clockwise me-2"></i>
                Try again
              </button>
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
                  <div className="page-pretitle">Overview</div>
                  <h2 className="page-title">Departments Management</h2>
                </div>
                <AddButton 
                  title="Add new department" 
                  onClick={() => openModal('add-member', null, refresh)}
                />
              </div>
            </div>
          </div>

          {/* Body */}
          <div className="page-body">
            <div className="container-xl">
              <div className="card">
                <CardHeader 
                  title="Department List" 
                  viewMode={viewMode} 
                  onViewModeChange={setViewMode} 
                />

                {/* Filters */}
                <TableFilters
                  perPage={pagination.perPage}
                  onPerPageChange={changePerPage}
                  searchValue={filters.search || ''}
                  onSearchChange={(value) => updateFilters({ search: value })}
                  searchPlaceholder="Search departments..."
                />

                {/* Empty State */}
                {departments.length === 0 && !loading && (
                  <div className="card-body">
                    <div className="empty">
                      <div className="empty-icon">
                        <i className="bi bi-people"></i>
                      </div>
                      <p className="empty-title">No departments found</p>
                      <p className="empty-subtitle text-secondary">
                        {filters.search 
                          ? 'Try adjusting your search' 
                          : 'Get started by adding a new department'
                        }
                      </p>
                    </div>
                  </div>
                )}

                {/* Grid View */}
                {viewMode === 'grid' && departments.length > 0 && (
                  <div className="card-body">
                    <div className="row row-cards">
                      {departments.map((department, index) => (
                        <div key={department.id} className="col-md-6 col-lg-4">
                          <div className="card card-sm">
                            <div className="card-body">
                              <div className="d-flex align-items-center mb-3">
                                <span className={`avatar avatar-lg rounded me-3 ${getAvatarColor(index)}`}>
                                  {getInitials(department.name)}
                                </span>
                                <div className="flex-fill">
                                  <div className="font-weight-medium">{department.name}</div>
                                  <div className="text-secondary small">{department.email}</div>
                                </div>
                              </div>
                            
                              <div className="mb-2">
                                {department.departments && department.departments.length > 0 ? (
                                  department.departments.map((department, idx) => (
                                    <span key={idx} className={`badge ${getDepartmentBadgeColor(department)} me-1`}>
                                      {department}
                                    </span>
                                  ))
                                ) : (
                                  <span className="badge bg-secondary-outline">No departments</span>
                                )}
                              </div>

                              <div className="btn-list">
                                <button 
                                  className="btn btn-sm btn-primary" 
                                  onClick={() => handleEditDepartment(department.id)}
                                >
                                  <i className="bi bi-pencil me-1"></i>
                                  Edit
                                </button>
                                <button 
                                  className="btn btn-sm btn-outline-danger" 
                                  onClick={() => handleDeleteDepartment(department.id)}
                                >
                                  <i className="bi bi-trash me-1"></i>
                                  Delete
                                </button>
                              </div>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
                  
                {/* Table View */}
                {viewMode === 'table' && departments.length > 0 && (
                  <div className="table-responsive">
                    <table className="table table-vcenter card-table table-striped">
                      <thead>
                        <tr>
                          <th>Department</th>
                          <th>Email</th>
                          <th>Departments</th>
                          <th className="w-1"></th>
                        </tr>
                      </thead>
                      <tbody>
                        {departments.map((department, index) => (
                          <tr key={department.id}>
                            <td>
                              <div className="d-flex py-1 align-items-center">
                                <span className={`avatar avatar-sm me-2 ${getAvatarColor(index)}`}>
                                  {getInitials(department.name)}
                                </span>
                                <div className="flex-fill">
                                  <div className="font-weight-medium">{department.name}</div>
                                </div>
                              </div>
                            </td>
                            <td className="text-secondary">{department.email}</td>
                            <td>
                              {department.departments && department.departments.length > 0 ? (
                                department.departments.map((department, idx) => (
                                  <span key={idx} className={`badge ${getDepartmentBadgeColor(department)} me-1`}>
                                    {department}
                                  </span>
                                ))
                              ) : (
                                <span className="badge bg-secondary-outline">No departments</span>
                              )}
                            </td>
                            <td>
                              <div className="btn-list flex-nowrap">
                                <button 
                                  className="btn btn-sm btn-icon btn-ghost-primary" 
                                  onClick={() => handleEditDepartment(department.id)}
                                  title="Edit department"
                                >
                                  <i className="bi bi-pencil"></i>
                                </button>
                                <button 
                                  className="btn btn-sm btn-icon btn-ghost-danger" 
                                  onClick={() => handleDeleteDepartment(department.id)} 
                                  title="Delete department"
                                >
                                  <i className="bi bi-trash"></i>
                                </button>
                              </div>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}

                {/* Pagination */}
                {departments.length > 0 && (
                  <Pagination 
                    pagination={pagination} 
                    onPageChange={goToPage} 
                  />
                )}
              </div>
            </div>
          </div>
        </div>
      </div> 
    </AuthLayout>
  );
}