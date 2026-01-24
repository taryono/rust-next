// backend/src/errors/app_error.rs
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use bcrypt::BcryptError;
use serde::Serialize;
use std::fmt;
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    // 400 Bad Request
    #[warn(unused_imports)]
    ValidationError(String),
    #[warn(unused_imports)]
    BadRequest(String),

    // 401 Unauthorized
    #[warn(unused_imports)]
    Unauthorized(String),
    #[warn(unused_imports)]
    InvalidToken(String),

    // 403 Forbidden
    #[warn(unused_imports)]
    Forbidden(String),

    // 404 Not Found
    #[warn(unused_imports)]
    NotFoundError(String),

    // 409 Conflict
    #[warn(unused_imports)]
    ConflictError(String),

    // 422 Unprocessable Entity
    #[warn(unused_imports)]
    UnprocessableEntity(String),

    // 500 Internal Server Error
    #[warn(unused_imports)]
    DatabaseError(String),
    #[warn(unused_imports)]
    InternalServerError(String),

    // External errors
    #[warn(unused_imports)]
    SeaORMError(sea_orm::DbErr),
    #[warn(unused_imports)]
    SerdeError(serde_json::Error),
    #[warn(unused_imports)]
    IOError(std::io::Error),

    // `AppError` needs to implement `From<jsonwebtoken::errors::Error>`

    // 500 Internal Server Error
    #[warn(unused_imports)]
    JWTError(jsonwebtoken::errors::Error),

    // 500 Internal Server Error
    #[warn(unused_imports)]
    ActixWebError(actix_web::Error),

    BcryptError(bcrypt::BcryptError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFoundError(msg) => write!(f, "Not found: {}", msg),
            AppError::ConflictError(msg) => write!(f, "Conflict: {}", msg),
            AppError::UnprocessableEntity(msg) => write!(f, "Unprocessable entity: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::SeaORMError(err) => write!(f, "Database error: {}", err),
            AppError::SerdeError(err) => write!(f, "Serialization error: {}", err),
            AppError::IOError(err) => write!(f, "IO error: {}", err),
            AppError::JWTError(err) => write!(f, "JWT error: {}", err),
            AppError::ActixWebError(err) => write!(f, "ActixWeb error: {}", err),
            AppError::BcryptError(err) => write!(f, "Bcrypt error: {}", err),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) | AppError::BadRequest(_) => StatusCode::BAD_REQUEST,

            AppError::Unauthorized(_) | AppError::InvalidToken(_) => StatusCode::UNAUTHORIZED,

            AppError::Forbidden(_) => StatusCode::FORBIDDEN,

            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,

            AppError::ConflictError(_) => StatusCode::CONFLICT,

            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,

            AppError::DatabaseError(_)
            | AppError::InternalServerError(_)
            | AppError::SeaORMError(_)
            | AppError::SerdeError(_)
            | AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JWTError(_) | AppError::ActixWebError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        let error_response = match self {
            // Client errors - show detailed message
            AppError::ValidationError(msg)
            | AppError::BadRequest(msg)
            | AppError::Unauthorized(msg)
            | AppError::InvalidToken(msg)
            | AppError::Forbidden(msg)
            | AppError::NotFoundError(msg)
            | AppError::ConflictError(msg)
            | AppError::UnprocessableEntity(msg) => ErrorResponse {
                error: msg.clone(),
                details: None,
            },

            // Server errors - hide details in production
            AppError::DatabaseError(msg) => {
                log::error!("Database error: {}", msg);
                ErrorResponse {
                    error: "A database error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(msg.clone())
                    } else {
                        None
                    },
                }
            }

            AppError::InternalServerError(msg) => {
                log::error!("Internal server error: {}", msg);
                ErrorResponse {
                    error: "An internal server error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(msg.clone())
                    } else {
                        None
                    },
                }
            }

            AppError::SeaORMError(err) => {
                log::error!("SeaORM error: {}", err);
                ErrorResponse {
                    error: "A database error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }

            AppError::SerdeError(err) => {
                log::error!("Serde error: {}", err);
                ErrorResponse {
                    error: "A serialization error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }

            AppError::IOError(err) => {
                log::error!("IO error: {}", err);
                ErrorResponse {
                    error: "An IO error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }

            AppError::JWTError(err) => {
                log::error!("JWT error: {}", err);
                ErrorResponse {
                    error: "A JWT error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }

            AppError::ActixWebError(err) => {
                log::error!("ActixWeb error: {}", err);
                ErrorResponse {
                    error: "An ActixWeb error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }

            AppError::BcryptError(err) => {
                log::error!("Bcrypt error: {}", err);
                ErrorResponse {
                    error: "A Bcrypt error occurred".to_string(),
                    details: if cfg!(debug_assertions) {
                        Some(err.to_string())
                    } else {
                        None
                    },
                }
            }
        };

        HttpResponse::build(status).json(error_response)
    }
}

// Implement From traits for automatic conversion
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::SeaORMError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerdeError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IOError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::JWTError(err)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::ActixWebError(err)
    }
}

impl From<BcryptError> for AppError {
    fn from(err: BcryptError) -> Self {
        // Jangan bocorin detail crypto ke client
        log::error!("Bcrypt error: {}", err);
        AppError::InternalServerError("Password verification failed".into())
    }
}

// Helper methods untuk create errors dengan builder pattern
impl AppError {
    #[warn(unused_imports)]
    pub fn validation<T: Into<String>>(msg: T) -> Self {
        AppError::ValidationError(msg.into())
    }
    #[warn(unused_imports)]
    pub fn bad_request<T: Into<String>>(msg: T) -> Self {
        AppError::BadRequest(msg.into())
    }
    #[warn(unused_imports)]
    pub fn unauthorized<T: Into<String>>(msg: T) -> Self {
        AppError::Unauthorized(msg.into())
    }
    #[warn(unused_imports)]
    pub fn forbidden<T: Into<String>>(msg: T) -> Self {
        AppError::Forbidden(msg.into())
    }
    #[warn(unused_imports)]
    pub fn not_found<T: Into<String>>(msg: T) -> Self {
        AppError::NotFoundError(msg.into())
    }
    #[warn(unused_imports)]
    pub fn conflict<T: Into<String>>(msg: T) -> Self {
        AppError::ConflictError(msg.into())
    }
    #[warn(unused_imports)]
    pub fn unprocessable_entity<T: Into<String>>(msg: T) -> Self {
        AppError::UnprocessableEntity(msg.into())
    }
    #[warn(unused_imports)]
    pub fn internal<T: Into<String>>(msg: T) -> Self {
        AppError::InternalServerError(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = AppError::NotFoundError("User not found".to_string());
        assert_eq!(error.to_string(), "Not found: User not found");
    }

    #[test]
    fn test_error_status_code() {
        assert_eq!(
            AppError::NotFoundError("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );

        assert_eq!(
            AppError::ValidationError("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
    }
}
