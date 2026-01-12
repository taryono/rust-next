'use client';

import { useEffect } from 'react';

export default function BootstrapClient() {
  useEffect(() => {
    require('bootstrap/dist/js/bootstrap.bundle.min.js');
  }, []);

  return null;
}
export default function CourseCarousel() {
  return (
    <div id="courseCarousel" className="carousel slide" data-bs-ride="carousel">
      <div className="carousel-indicators">
        <button type="button" data-bs-target="#courseCarousel" data-bs-slide-to="0" className="active"></button>
        <button type="button" data-bs-target="#courseCarousel" data-bs-slide-to="1"></button>
        <button type="button" data-bs-target="#courseCarousel" data-bs-slide-to="2"></button>
      </div>
      
      <div className="carousel-inner">
        <div className="carousel-item active">
          <img src="https://placehold.co/1200x400/667eea/white?text=Featured+Course+1" className="d-block w-100" alt="Course 1" />
          <div className="carousel-caption">
            <h3>Mahir Berbicara Bahasa Inggris</h3>
            <p>Dengan aksen native by Pintro</p>
            <a href="#" className="btn btn-primary">Lihat Detail</a>
          </div>
        </div>
        <div className="carousel-item">
          <img src="https://placehold.co/1200x400/764ba2/white?text=Featured+Course+2" className="d-block w-100" alt="Course 2" />
          <div className="carousel-caption">
            <h3>Training HTML CSS</h3>
            <p>Sertifikasi untuk Keuangan</p>
            <a href="#" className="btn btn-primary">Lihat Detail</a>
          </div>
        </div>
        <div className="carousel-item">
          <img src="https://placehold.co/1200x400/8b5cf6/white?text=Featured+Course+3" className="d-block w-100" alt="Course 3" />
          <div className="carousel-caption">
            <h3>Python Programming</h3>
            <p>Complete Bootcamp</p>
            <a href="#" className="btn btn-primary">Lihat Detail</a>
          </div>
        </div>
      </div>
      
      <button className="carousel-control-prev" type="button" data-bs-target="#courseCarousel" data-bs-slide="prev">
        <span className="carousel-control-prev-icon"></span>
      </button>
      <button className="carousel-control-next" type="button" data-bs-target="#courseCarousel" data-bs-slide="next">
        <span className="carousel-control-next-icon"></span>
      </button>
    </div>
  );
}