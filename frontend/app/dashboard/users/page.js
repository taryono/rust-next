'use client';

import { useState, useEffect } from 'react';
import AuthLayout from '@/components/layout/AuthLayout';
import { api } from '@/lib/api';
import { alertError,alertConfirm,alertSuccess } from '@/lib/alert';

export default function Users() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [filterRole, setFilterRole] = useState('all');
  const [viewMode, setViewMode] = useState('grid');
  
  // Pagination states
  const [currentPage, setCurrentPage] = useState(1);
  const [perPage, setPerPage] = useState(10);
  const [totalPages, setTotalPages] = useState(1);
  const [total, setTotal] = useState(0);

  useEffect(() => {
    fetchUsers();
  }, [currentPage, perPage, searchQuery, filterRole]);

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const params = new URLSearchParams({
        page: currentPage.toString(),
        per_page: perPage.toString(),
      });
      
      if (searchQuery) params.append('search', searchQuery);
      if (filterRole !== 'all') params.append('role', filterRole);
      
      const response = await api.getUsers(`?${params.toString()}`);
      const data = response.data || response;
      
      setUsers(data.users || []);
      setTotal(data.total || 0);
      setTotalPages(data.total_pages || 1);
      
    } catch (err) {
      console.error('Error:', err);
      alertError('Failed to fetch users');
    } finally {
      setLoading(false);
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

  const handlePageChange = (page) => {
    if (page >= 1 && page <= totalPages) {
      setCurrentPage(page);
    }
  };

  const getPageNumbers = () => {
    const pages = [];
    const maxVisible = 5;
    
    if (totalPages <= maxVisible) {
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      if (currentPage <= 3) {
        for (let i = 1; i <= 4; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      } else if (currentPage >= totalPages - 2) {
        pages.push(1);
        pages.push('...');
        for (let i = totalPages - 3; i <= totalPages; i++) pages.push(i);
      } else {
        pages.push(1);
        pages.push('...');
        for (let i = currentPage - 1; i <= currentPage + 1; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      }
    }
    
    return pages;
  };

  if (loading && users.length === 0) {
    return (
      <AuthLayout>
        <div className="page">
          <div className="page-wrapper">
            <div className="container-xl d-flex flex-column justify-content-center" style={{minHeight: '100vh'}}>
              <div className="text-center">
                <div className="spinner-border text-primary" role="status"></div>
                <p className="mt-3">Loading users...</p>
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
                  <div className="page-pretitle">Overview</div>
                  <h2 className="page-title">Users Management</h2>
                </div>
                
                <div className="col-auto ms-auto d-print-none">
                  <div className="btn-list">
                    <button className="btn btn-primary d-none d-sm-inline-block">
                      <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
                        <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                        <path d="M12 5l0 14" />
                        <path d="M5 12l14 0" />
                      </svg>
                      Add new user
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Page body */}
          <div className="page-body">
            <div className="container-xl">
              {/* Stats cards */}
              <div className="row row-deck row-cards mb-3">
                <div className="col-sm-6 col-lg-3">
                  <div className="card">
                    <div className="card-body">
                      <div className="d-flex align-items-center">
                        <div className="subheader">Total Users</div>
                      </div>
                      <div className="h1 mb-3">{total}</div>
                    </div>
                  </div>
                </div>
                
                <div className="col-sm-6 col-lg-3">
                  <div className="card">
                    <div className="card-body">
                      <div className="d-flex align-items-center">
                        <div className="subheader">Current Page</div>
                      </div>
                      <div className="h1 mb-3">{currentPage} / {totalPages}</div>
                    </div>
                  </div>
                </div>
                
                <div className="col-sm-6 col-lg-3">
                  <div className="card">
                    <div className="card-body">
                      <div className="d-flex align-items-center">
                        <div className="subheader">Showing</div>
                      </div>
                      <div className="h1 mb-3">{users.length}</div>
                    </div>
                  </div>
                </div>
                
                <div className="col-sm-6 col-lg-3">
                  <div className="card">
                    <div className="card-body">
                      <div className="d-flex align-items-center">
                        <div className="subheader">Per Page</div>
                      </div>
                      <div className="h1 mb-3">{perPage}</div>
                    </div>
                  </div>
                </div>
              </div>

              {/* Main card */}
              <div className="card">
                <div className="card-header">
                  <h3 className="card-title">Users List</h3>
                  
                  <div className="ms-auto">
                    <div className="btn-group" role="group">
                      <button 
                        type="button" 
                        className={`btn btn-sm ${viewMode === 'grid' ? 'btn-primary' : 'btn-outline-primary'}`}
                        onClick={() => setViewMode('grid')}
                      >
                        Grid
                      </button>
                      <button 
                        type="button" 
                        className={`btn btn-sm ${viewMode === 'table' ? 'btn-primary' : 'btn-outline-primary'}`}
                        onClick={() => setViewMode('table')}
                      >
                        Table
                      </button>
                    </div>
                  </div>
                </div>
                
                <div className="card-body border-bottom py-3">
                  <div className="d-flex">
                    <div className="text-secondary">
                      Show
                      <div className="mx-2 d-inline-block">
                        <select 
                          className="form-select form-select-sm" 
                          value={perPage}
                          onChange={(e) => {
                            setPerPage(Number(e.target.value));
                            setCurrentPage(1);
                          }}
                        >
                          <option value="5">5</option>
                          <option value="10">10</option>
                          <option value="20">20</option>
                          <option value="50">50</option>
                        </select>
                      </div>
                      entries
                    </div>
                    
                    <div className="ms-auto text-secondary">
                      <div className="d-flex gap-2">
                        <select 
                          className="form-select form-select-sm" 
                          value={filterRole}
                          onChange={(e) => {
                            setFilterRole(e.target.value);
                            setCurrentPage(1);
                          }}
                        >
                          <option value="all">All roles</option>
                          <option value="admin">Admin</option>
                          <option value="editor">Editor</option>
                          <option value="viewer">Viewer</option>
                        </select>
                        
                        <div className="input-icon">
                          <span className="input-icon-addon">
                            <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
                              <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                              <path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" />
                              <path d="M21 21l-6 -6" />
                            </svg>
                          </span>
                          <input 
                            type="text" 
                            className="form-control form-control-sm" 
                            placeholder="Search users..."
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                          />
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {loading ? (
                  <div className="card-body text-center py-5">
                    <div className="spinner-border text-primary" role="status"></div>
                  </div>
                ) : users.length === 0 ? (
                  <div className="card-body text-center py-5">
                    <div className="empty">
                      <div className="empty-icon">
                        <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
                          <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                          <path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />
                          <path d="M9 10l.01 0" />
                          <path d="M15 10l.01 0" />
                          <path d="M9.5 15a3.5 3.5 0 0 0 5 0" />
                        </svg>
                      </div>
                      <p className="empty-title">No users found</p>
                      <p className="empty-subtitle text-secondary">
                        Try adjusting your search or filter to find what you're looking for.
                      </p>
                    </div>
                  </div>
                ) : (
                  <>
                    {/* Grid View */}
                    {viewMode === 'grid' && (
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
                                      <span className="badge bg-secondary">No roles</span>
                                    )}
                                  </div>
                                </div>
                              </div>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}

                    {/* Table View */}
                    {viewMode === 'table' && (
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
                                    <span className="badge bg-secondary">No roles</span>
                                  )}
                                </td>
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
                  </>
                )}

                {/* Pagination */}
                <div className="card-footer d-flex align-items-center">
                  <p className="m-0 text-secondary">
                    Showing <span>{((currentPage - 1) * perPage) + 1}</span> to{' '}
                    <span>{Math.min(currentPage * perPage, total)}</span> of{' '}
                    <span>{total}</span> entries
                  </p>
                  <ul className="pagination m-0 ms-auto">
                    <li className={`page-item ${currentPage === 1 ? 'disabled' : ''}`}>
                      <a 
                        className="page-link" 
                        href="#" 
                        onClick={(e) => {
                          e.preventDefault();
                          handlePageChange(currentPage - 1);
                        }}
                      >
                        prev
                      </a>
                    </li>
                    
                    {getPageNumbers().map((page, idx) => (
                      page === '...' ? (
                        <li key={`ellipsis-${idx}`} className="page-item disabled">
                          <span className="page-link">...</span>
                        </li>
                      ) : (
                        <li key={page} className={`page-item ${currentPage === page ? 'active' : ''}`}>
                          <a 
                            className="page-link" 
                            href="#"
                            onClick={(e) => {
                              e.preventDefault();
                              handlePageChange(page);
                            }}
                          >
                            {page}
                          </a>
                        </li>
                      )
                    ))}
                    
                    <li className={`page-item ${currentPage === totalPages ? 'disabled' : ''}`}>
                      <a 
                        className="page-link" 
                        href="#"
                        onClick={(e) => {
                          e.preventDefault();
                          handlePageChange(currentPage + 1);
                        }}
                      >
                        next
                      </a>
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </AuthLayout>
  );
}