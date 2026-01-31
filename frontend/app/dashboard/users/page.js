// frontend/app/dashboard/users/page.js
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
import ModalManager from '@/components/modals/ModalManager';


export default function Users() {
  const { openModal } = useModalStore(); 
  const [viewMode, setViewMode] = useState('grid');
  const { handleUnauthorized } = useAuth();

  // Setup unauthorized handler on mount
  useEffect(() => {
    setUnauthorizedHandler(handleUnauthorized);
    
    return () => {
      setUnauthorizedHandler(null);
    };
  }, [handleUnauthorized]);
  // Gunakan hook pagination (ini sudah handle semuanya)
  const {
    data: users,
    loading,
    error,
    pagination,
    filters,
    goToPage,
    changePerPage,
    updateFilters,
    refresh, // tambahkan ini untuk refresh data
  } = usePagination(api.getUsers);

  // Helper functions
  const getRoleBadgeColor = (role) => {
    const colors = {
      'Admin': 'bg-red',
      'Editor': 'bg-blue',
      'Viewer': 'bg-green',
      'default': 'bg-gray'
    };
    return colors[role] || colors.default;
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

  // Handle edit user
  const handleEditUser = (userId) => {
    const user = users.find(u => u.id === userId);
    openModal('add-member', user, refresh);
  };

  // Handle delete user
  const handleDeleteUser = async (userId) => {
    const result = await showConfirm(
      'This user will be permanently deleted.',
      'Delete User?'
    );
    
    if (result.isConfirmed) {
      try {
        await api.deleteUser(userId); // pastikan ada di api.js
        alertSuccess('User deleted successfully!');
        refresh(); // refresh data setelah delete
      } catch (error) {
        alertError(error.response?.data?.message || 'Failed to delete user');
      }
    }
  }; 

  // Loading state
  if (loading && users.length === 0) {
    return (
      <AuthLayout>
        <Loader title="Loading Users..." /> 
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
            <p className="empty-title">Failed to load users</p>
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
                  <h2 className="page-title">Users Management</h2>
                </div>
                <AddButton 
                  title="Add new user" 
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
                  title="User List" 
                  viewMode={viewMode} 
                  onViewModeChange={setViewMode} 
                />

                {/* Filters */}
                <TableFilters
                  perPage={pagination.perPage}
                  onPerPageChange={changePerPage}
                  searchValue={filters.search || ''}
                  onSearchChange={(value) => updateFilters({ search: value })}
                  searchPlaceholder="Search users..."
                />

                {/* Empty State */}
                {users.length === 0 && !loading && (
                  <div className="card-body">
                    <div className="empty">
                      <div className="empty-icon">
                        <i className="bi bi-people"></i>
                      </div>
                      <p className="empty-title">No users found</p>
                      <p className="empty-subtitle text-secondary">
                        {filters.search 
                          ? 'Try adjusting your search' 
                          : 'Get started by adding a new user'
                        }
                      </p>
                    </div>
                  </div>
                )}

                {/* Grid View */}
                {viewMode === 'grid' && users.length > 0 && (
                  <div className="card-body">
                    <div className="row row-cards">
                      {users.map((user, index) => (
                        <div key={user.id} className="col-md-6 col-lg-4">
                          <div className="card card-sm">
                            <div className="card-body">
                              <div className="d-flex align-items-center mb-3">
                                <span className={`avatar avatar-lg rounded me-3 ${getAvatarColor(index)}`}>
                                  {getInitials(user.name)}
                                </span>
                                <div className="flex-fill">
                                  <div className="font-weight-medium">{user.name}</div>
                                  <div className="text-secondary small">{user.email}</div>
                                </div>
                              </div>
                            
                              <div className="mb-2">
                                {user.roles && user.roles.length > 0 ? (
                                  user.roles.map((role, idx) => (
                                    <span key={idx} className={`badge ${getRoleBadgeColor(role)} me-1`}>
                                      {role}
                                    </span>
                                  ))
                                ) : (
                                  <span className="badge bg-secondary-outline">No roles</span>
                                )}
                              </div>

                              <div className="btn-list">
                                <button 
                                  className="btn btn-sm btn-primary" 
                                  onClick={() => handleEditUser(user.id)}
                                >
                                  <i className="bi bi-pencil me-1"></i>
                                  Edit
                                </button>
                                <button 
                                  className="btn btn-sm btn-outline-danger" 
                                  onClick={() => handleDeleteUser(user.id)}
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
                {viewMode === 'table' && users.length > 0 && (
                  <div className="table-responsive">
                    <table className="table table-vcenter card-table table-striped">
                      <thead>
                        <tr>
                          <th>User</th>
                          <th>Email</th>
                          <th>Roles</th>
                          <th className="w-1"></th>
                        </tr>
                      </thead>
                      <tbody>
                        {users.map((user, index) => (
                          <tr key={user.id}>
                            <td>
                              <div className="d-flex py-1 align-items-center">
                                <span className={`avatar avatar-sm me-2 ${getAvatarColor(index)}`}>
                                  {getInitials(user.name)}
                                </span>
                                <div className="flex-fill">
                                  <div className="font-weight-medium">{user.name}</div>
                                </div>
                              </div>
                            </td>
                            <td className="text-secondary">{user.email}</td>
                            <td>
                              {user.roles && user.roles.length > 0 ? (
                                user.roles.map((role, idx) => (
                                  <span key={idx} className={`badge ${getRoleBadgeColor(role)} me-1`}>
                                    {role}
                                  </span>
                                ))
                              ) : (
                                <span className="badge bg-secondary-outline">No roles</span>
                              )}
                            </td>
                            <td>
                              <div className="btn-list flex-nowrap">
                                <button 
                                  className="btn btn-sm btn-icon btn-ghost-primary" 
                                  onClick={() => handleEditUser(user.id)}
                                  title="Edit user"
                                >
                                  <i className="bi bi-pencil"></i>
                                </button>
                                <button 
                                  className="btn btn-sm btn-icon btn-ghost-danger" 
                                  onClick={() => handleDeleteUser(user.id)} 
                                  title="Delete user"
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
                {users.length > 0 && (
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