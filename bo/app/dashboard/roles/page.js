'use client';

import { useState, useEffect } from 'react';
import AuthLayout from '@/components/layout/AuthLayout';
import { api } from '@/lib/api';
import { alertError,alertConfirm,alertSuccess } from '@/lib/alert';
import { usePagination } from '@/hooks/usePagination';
import Pagination from '@/components/common/Pagination'; 
import useModalStore from '@/store/modalStore';
import CardHeader from '@/components/ui/CardHeader';
import Loader from '@/components/ui/Loader';
import AddButton from '@/components/ui/AddButton';
import TableFilters from '@/components/ui/TableFilters';

export default function Roles() {
  const { openModal } = useModalStore();
  const {
    data: roles,
    loading,
    error,
    pagination,
    filters,
    goToPage,
    changePerPage,
    updateFilters,
    refresh,
  } = usePagination(api.getRoles);
 
  const [searchQuery, setSearchQuery] = useState('');
  const [filterRole, setFilterRole] = useState('all');
  const [viewMode, setViewMode] = useState('grid');
  const [isLoading, setIsLoading] = useState(false);
  // Pagination states
  const [currentPage, setCurrentPage] = useState(1);
  const [perPage, setPerPage] = useState(10);
  const [totalPages, setTotalPages] = useState(1);
  const [total, setTotal] = useState(0); 

  useEffect(() => {
    fetchRoles();
  }, [currentPage, perPage, searchQuery, filterRole]);

  const fetchRoles = async () => {
    try { 
       if (isLoading) return; // Prevent multiple calls  
      setIsLoading(true);
      const params = new URLSearchParams({
        page: currentPage.toString(),
        per_page: perPage.toString(),
      });
      
      if (searchQuery) params.append('search', searchQuery);
      if (filterRole !== 'all') params.append('role', filterRole);
      
      const response = await api.getRoles(`?${params.toString()}`);
      const data = response.data || response; 
      setTotal(data.total || 0);
      setTotalPages(data.total_pages || 1);
      
    } catch (err) {
      console.error('Error:', err);
      alertError('Failed to fetch roles');
    } finally { 
    }
  };

  // Debounce search
  useEffect(() => {
    const timer = setTimeout(() => {
      setCurrentPage(1); // Reset to page 1 on search
    }, 500);
    
    return () => clearTimeout(timer);
  }, [searchQuery]);

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
    const colors = ['bg-blue-lt', 'bg-azure-lt', 'bg-indigo-lt', 'bg-purple-lt', 
                   'bg-pink-lt', 'bg-red-lt', 'bg-orange-lt', 'bg-yellow-lt'];
    return colors[index % colors.length];
  };

  const getInitials = (name) => {
    return name?.split(' ').map(n => n[0]).join('').toUpperCase() || '??';
  }; 

  if (loading && roles.length === 0) {
    return (
      <AuthLayout>
        <Loader title={"Loading Roles...."} /> 
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
                  <h2 className="page-title">Roles Management</h2>
                </div>
                
                <AddButton 
                  title="Add new role" 
                  onClick={() => openModal('add-member', null, refresh)}
                />
              </div>
            </div>
          </div>
          {/* Body */}
          <div className="page-body">
            <div className="container-xl">
              <div className="card">
                <CardHeader title={"Role List"} viewMode={viewMode} onViewModeChange={setViewMode} />
 
                  {/* Filters */}
                  <TableFilters
                    perPage={pagination.perPage}
                    onPerPageChange={changePerPage}
                    searchValue={filters.search || ''}
                    onSearchChange={(value) => updateFilters({ search: value })}
                    searchPlaceholder="Search roles..."
                  />

                  {viewMode === 'grid' && (
                      <div className="card-body">
                        <div className="row row-cards">
                          {roles.map((role, index) => (
                            <div key={role.id} className="col-md-6 col-lg-4">
                              <div className="card card-sm">
                                <div className="card-body">
                                  <div className="d-flex align-items-center mb-3">
                                    <span className={`avatar avatar-lg rounded me-3 ${getAvatarColor(index)}`}>
                                      {getInitials(role.code)}
                                    </span>
                                    <div className="flex-fill"> 
                                      <div className="font-weight-medium">{role.name}</div>
                                      <div className="text-secondary small">{role.description}</div>
                                    </div>
                                  </div> 
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                      
                    {viewMode === 'table' && (
                      <div className="table-responsive">
                        <table className="table table-vcenter card-table table-striped">
                          <thead>
                            <tr>
                              <th>Code</th>
                              <th>Name</th>
                              <th>Description</th> 
                              <th className="w-1"></th>
                            </tr>
                          </thead>
                          <tbody>
                            {roles.map((role, index) => (
                              <tr key={role.id}>
                                <td className="text-secondary">{role.code}</td>
                                <td>
                                  <div className="d-flex py-1 align-items-center">
                                    <span className={`avatar avatar-sm me-2 ${getAvatarColor(index)}`}>
                                      {getInitials(role.name)}
                                    </span>
                                    <div className="flex-fill">
                                      <div className="font-weight-medium">{role.name}</div>
                                    </div>
                                  </div>
                                </td>
                                <td className="text-secondary">{role.description}</td>
                                 
                                <td>
                                  <div className="btn-list flex-nowrap">
                                    <button className="btn btn-sm btn-icon btn-ghost-primary">
                                      Edit
                                    </button>
                                  </div>
                                </td>
                              </tr>
                            ))}
                          </tbody>
                        </table>
                      </div>
                    )}  


                  {/* Pagination Component */}
                  <Pagination 
                    pagination={pagination} 
                    onPageChange={goToPage} 
                  />

                </div>
              </div>
            </div>
          </div>
        </div> 
    </AuthLayout>
  );
}