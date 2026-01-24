'use client'; 
import { useState, useEffect, use } from 'react';
export default function TableHeader({ title }) {  
const [viewMode, setViewMode] = useState('grid'); 
  return (
    <>
      <div className="card-header">
        <h3 className="card-title">{title}</h3>
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
    </>
  );
}