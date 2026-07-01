'use client'; 
import { useState, useEffect } from 'react';

export default function Loader({ title}) {  
    const [minHeight, setMinHeight] = useState();
 
    useEffect(() => {
        setMinHeight();
    }, [minHeight]);

  return (
    <>
       <div className="page">
          <div className="page-wrapper">
            <div className="container-xl d-flex flex-column justify-content-center" style={{minHeight: '100vh'}}>
              <div className="text-center">
                <div className="spinner-border text-white" role="status"></div>
                  <div>
                    <span className='text-white font-size-32'>{title}... </span>
                  </div>
                  
              </div>
            </div>
          </div>
        </div>
    </>
  );
}