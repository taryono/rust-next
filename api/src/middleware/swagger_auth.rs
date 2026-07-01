// src/middleware/swagger_auth.rs
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let username = std::env::var("SWAGGER_USERNAME").unwrap_or("admin".to_string());
    let password = std::env::var("SWAGGER_PASSWORD").unwrap_or("secret".to_string());

    if credentials.user_id() == username
        && credentials
            .password()
            .map(|p| p == &password)
            .unwrap_or(false)
    {
        Ok(req)
    } else {
        log::info!("Unauthorized request");

        Err((actix_web::error::ErrorUnauthorized("Unauthorized"), req))
    }
}
