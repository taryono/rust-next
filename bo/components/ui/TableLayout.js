'use client';
  
import useUIStore from '@/store/uiStore'; 


export default function TableLayout({ children }) {
  const { sidebarCollapsed } = useUIStore();
  
  return (
    <>
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
                    <button className="btn btn-primary d-none d-sm-inline-block" onClick={()=> openModal('add-member',null)}>
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
          {/* Body */}
          <div className="page-body">
            <div className="container-xl">
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

                  {/* Filters */}
                  <div className="card-body border-bottom py-3">
                    <div className="d-flex">
                      <div className="text-secondary">
                        Show
                        <div className="mx-2 d-inline-block">
                          <select 
                            className="form-select form-select-sm" 
                            value={pagination.perPage}
                            onChange={(e) => changePerPage(Number(e.target.value))}
                          >
                            <option value="5">5</option>
                            <option value="10">10</option>
                            <option value="20">20</option>
                            <option value="50">50</option>
                          </select>
                        </div>
                        entries
                      </div>
                      
                      <div className="ms-auto">
                        <input 
                          type="text" 
                          className="form-control form-control-sm" 
                          placeholder="Search users..."
                          value={filters.search}
                          onChange={(e) => updateFilters({ search: e.target.value })}
                        />
                      </div>
                    </div>
                  </div>

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
                                      <span className="badge bg-primary-outline">No roles</span>
                                    )}
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
    </>
  );
}