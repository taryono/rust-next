export default function Pagination({ pagination, onPageChange }) {
  const { page, totalPages, hasPrev, hasNext, total, perPage } = pagination;

  const getPageNumbers = () => {
    const pages = [];
    const maxVisible = 5;
    
    if (totalPages <= maxVisible) {
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      if (page <= 3) {
        for (let i = 1; i <= 4; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      } else if (page >= totalPages - 2) {
        pages.push(1);
        pages.push('...');
        for (let i = totalPages - 3; i <= totalPages; i++) pages.push(i);
      } else {
        pages.push(1);
        pages.push('...');
        for (let i = page - 1; i <= page + 1; i++) pages.push(i);
        pages.push('...');
        pages.push(totalPages);
      }
    }
    
    return pages;
  };

  const startItem = ((page - 1) * perPage) + 1;
  const endItem = Math.min(page * perPage, total);

  return (
    <div className="card-footer d-flex align-items-center">
      <p className="m-0 text-secondary">
        Showing <span>{startItem}</span> to <span>{endItem}</span> of <span>{total}</span> entries
      </p>
      <ul className="pagination m-0 ms-auto">
        <li className={`page-item ${!hasPrev ? 'disabled' : ''}`}>
          <a 
            className="page-link" 
            href="#" 
            onClick={(e) => {
              e.preventDefault();
              if (hasPrev) onPageChange(page - 1);
            }}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
              <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
              <path d="M15 6l-6 6l6 6" />
            </svg>
            prev
          </a>
        </li>
        
        {getPageNumbers().map((pageNum, idx) => (
          pageNum === '...' ? (
            <li key={`ellipsis-${idx}`} className="page-item disabled">
              <span className="page-link">...</span>
            </li>
          ) : (
            <li key={pageNum} className={`page-item ${page === pageNum ? 'active' : ''}`}>
              <a 
                className="page-link" 
                href="#"
                onClick={(e) => {
                  e.preventDefault();
                  onPageChange(pageNum);
                }}
              >
                {pageNum}
              </a>
            </li>
          )
        ))}
        
        <li className={`page-item ${!hasNext ? 'disabled' : ''}`}>
          <a 
            className="page-link" 
            href="#"
            onClick={(e) => {
              e.preventDefault();
              if (hasNext) onPageChange(page + 1);
            }}
          >
            next
            <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
              <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
              <path d="M9 6l6 6l-6 6" />
            </svg>
          </a>
        </li>
      </ul>
    </div>
  );
}