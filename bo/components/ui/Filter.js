'use client';  
import { useState, useEffect } from 'react';

export default function Filter({ pagination ,filters,  changePerPage, updateFilters }) {  
  const [search, setSearch] = useState('');

  useEffect(()=> {
    setSearch('');
  }, [search])
  return (
    <>
       <div className="card-body border-bottom py-3">
          <div className="d-flex">
              <div className="text-secondary">
                Show
                <div className="mx-2 d-inline-block">
                    <select 
                    className="form-select form-select-sm" 
                    value={pagination.perPage}
                    onChange={changePerPage}
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
                    placeholder="Search units..."
                    value={filters.search}
                    onChange={updateFilters}
                />
              </div>
          </div>
        </div>
    </>
  );
}