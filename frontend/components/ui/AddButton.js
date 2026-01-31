// frontend/components/ui/AddButton.js
'use client'; 

export default function AddButton({ title , onClick }) {  
 
  return (
    <>
    <div className="col-auto ms-auto d-print-none">
        <div className="btn-list">
            <button className="btn btn-primary d-none d-sm-inline-block"  onClick={onClick}>
            <svg xmlns="http://www.w3.org/2000/svg" className="icon" width="24" height="24" viewBox="0 0 24 24" strokeWidth="2" stroke="currentColor" fill="none" strokeLinecap="round" strokeLinejoin="round">
                <path stroke="none" d="M0 0h24v24H0z" fill="none"/>
                <path d="M12 5l0 14" />
                <path d="M5 12l14 0" />
            </svg>
             {title}
            </button>
        </div>
    </div>
    </>
  );
}