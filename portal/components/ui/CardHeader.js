'use client'; 

export default function CardHeader({ title , viewMode, onViewModeChange }) {  
 
  return (
    <>
      <div className="card-header">
        <h3 className="card-title">{title}</h3>
        <div className="ms-auto">
            <div className="btn-group" role="group">
            <button 
                type="button" 
                className={`btn btn-sm ${viewMode === 'grid' ? 'btn-primary' : 'btn-outline-primary'}`}
                onClick={() => onViewModeChange('grid')}
            >
                Grid
            </button>
            <button 
                type="button" 
                className={`btn btn-sm ${viewMode === 'table' ? 'btn-primary' : 'btn-outline-primary'}`}
                onClick={() => onViewModeChange('table')}
            >
                Table
            </button>
            </div>
        </div>
    </div>
    </>
  );
}