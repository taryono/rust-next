export default function CourseCard({ 
  image, 
  category, 
  title, 
  provider, 
  price, 
  rating = 0,
  badge = null,
  badgeColor = "primary" 
}) {
  return (
    <div className="col-12 col-md-6 col-lg-4 mb-4">
      <div className="card course-card h-100 border-0 shadow-sm">
        <div className="position-relative">
          <img 
            src={image} 
            className="card-img-top course-image" 
            alt={title}
          />
          {badge && (
            <span className={`position-absolute top-0 start-0 m-2 badge bg-${badgeColor} course-badge`}>
              {badge}
            </span>
          )}
        </div>
        <div className="card-body d-flex flex-column">
          <small className="text-muted mb-2">{category}</small>
          <h5 className="card-title course-title mb-3">{title}</h5>
          <div className="mt-auto">
            {rating > 0 && (
              <div className="mb-2">
                {[...Array(5)].map((_, i) => (
                  <i 
                    key={i} 
                    className={`bi bi-star${i < rating ? '-fill' : ''} text-warning`}
                  ></i>
                ))}
              </div>
            )}
            <div className="d-flex justify-content-between align-items-center">
              <small className="text-muted">{provider}</small>
              <span className="fw-bold text-primary">Rp {price.toLocaleString('id-ID')}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}