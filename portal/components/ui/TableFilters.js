// frontend/components/ui/TableFilters.js
'use client';

export default function TableFilters({ 
  perPage, 
  onPerPageChange, 
  searchValue, 
  onSearchChange,
  searchPlaceholder = "Search...",
  perPageOptions = [5, 10, 20, 50] // bisa di-customize
}) {
  return (
    <div className="card-body border-bottom py-3">
      <div className="d-flex">
        <div className="text-secondary">
          Show
          <div className="mx-2 d-inline-block">
            <select 
              className="form-select form-select-sm" 
              value={perPage}
              onChange={(e) => onPerPageChange(Number(e.target.value))}
            >
              {perPageOptions.map(option => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </select>
          </div>
          entries
        </div>
        
        <div className="ms-auto">
          <input 
            type="text" 
            className="form-control form-control-sm" 
            placeholder={searchPlaceholder}
            value={searchValue}
            onChange={(e) => onSearchChange(e.target.value)}
          />
        </div>
      </div>
    </div>
  );
}