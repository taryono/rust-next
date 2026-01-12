'use client';

import CourseCard from '@/components/ui/CourseCard';

export default function CoursesPage() {
  const courses = [
    {
      id: 1,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'Mahir berbicara Bahasa Inggris dengan aksen native by Pintro',
      provider: 'Harga mulai dari',
      price: 150000,
      rating: 0,
      badge: 'pintro',
      badgeColor: 'warning'
    },
    {
      id: 2,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'Training dan Sertifikasi HTML CSS Keuangan',
      provider: 'Harga mulai dari',
      price: 150000,
      rating: 5,
      badge: 'TRAINING & SERTIFIKASI',
      badgeColor: 'info'
    },
    {
      id: 3,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'Graphic Design Mastery - Photoshop, Illustrator, InDesign',
      provider: 'Harga mulai dari',
      price: 200000,
      rating: 5,
      badge: 'POPULER',
      badgeColor: 'danger'
    },
    {
      id: 4,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'Digital Marketing untuk Pemula - SEO, SEM, Social Media',
      provider: 'Harga mulai dari',
      price: 175000,
      rating: 4,
      badge: 'BEST SELLER',
      badgeColor: 'success'
    },
    {
      id: 5,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'Python Programming Complete Bootcamp',
      provider: 'Harga mulai dari',
      price: 250000,
      rating: 5,
      badge: 'NEW',
      badgeColor: 'primary'
    },
    {
      id: 6,
      image: 'https://placehold.co/400x200/FF9933/white?text=English+Course',
      category: 'Yayasan Edukasi Sejahtera',
      title: 'UI/UX Design Professional dengan Figma',
      provider: 'Harga mulai dari',
      price: 180000,
      rating: 5,
      badge: null,
      badgeColor: 'primary'
    },
  ];

  return (
    <div className="courses-page">
      {/* Header */}
      <div className="courses-header">
        <div className="container py-5">
          <nav aria-label="breadcrumb">
            <ol className="breadcrumb">
              <li className="breadcrumb-item">
                <a href="/" className="text-decoration-none">
                  <i className="bi bi-house-door me-1"></i>
                  Home
                </a>
              </li>
              <li className="breadcrumb-item active" aria-current="page">
                Acara lainnya
              </li>
            </ol>
          </nav>
          <h1 className="display-5 fw-bold mb-4">Acara lainnya</h1>
          
          {/* Filters */}
          <div className="row g-3 mb-4">
            <div className="col-md-3">
              <div className="dropdown">
                <button 
                  className="btn btn-outline-secondary w-100 dropdown-toggle" 
                  type="button" 
                  data-bs-toggle="dropdown"
                >
                  <i className="bi bi-funnel me-2"></i>
                  Urutkan
                </button>
                <ul className="dropdown-menu">
                  <li><a className="dropdown-item" href="#">Terbaru</a></li>
                  <li><a className="dropdown-item" href="#">Terpopuler</a></li>
                  <li><a className="dropdown-item" href="#">Harga Terendah</a></li>
                  <li><a className="dropdown-item" href="#">Harga Tertinggi</a></li>
                </ul>
              </div>
            </div>
            <div className="col-md-3">
              <div className="dropdown">
                <button 
                  className="btn btn-outline-secondary w-100 dropdown-toggle" 
                  type="button" 
                  data-bs-toggle="dropdown"
                >
                  <i className="bi bi-grid me-2"></i>
                  Penyelenggara
                </button>
                <ul className="dropdown-menu">
                  <li><a className="dropdown-item" href="#">Semua</a></li>
                  <li><a className="dropdown-item" href="#">Yayasan Edukasi</a></li>
                  <li><a className="dropdown-item" href="#">Pintro</a></li>
                </ul>
              </div>
            </div>
            <div className="col-md-6">
              <div className="input-group">
                <span className="input-group-text bg-white">
                  <i className="bi bi-search"></i>
                </span>
                <input 
                  type="text" 
                  className="form-control" 
                  placeholder="Cari kursus..."
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Courses Grid */}
      <div className="container pb-5">
        <div className="row">
          {courses.map(course => (
            <CourseCard key={course.id} {...course} />
          ))}
        </div>

        {/* Pagination */}
        <nav aria-label="Course pagination" className="mt-5">
          <ul className="pagination justify-content-center">
            <li className="page-item disabled">
              <a className="page-link" href="#" tabIndex="-1">Previous</a>
            </li>
            <li className="page-item active"><a className="page-link" href="#">1</a></li>
            <li className="page-item"><a className="page-link" href="#">2</a></li>
            <li className="page-item"><a className="page-link" href="#">3</a></li>
            <li className="page-item">
              <a className="page-link" href="#">Next</a>
            </li>
          </ul>
        </nav>
      </div>
    </div>
  );
}