// src/context/service_context.rs
use crate::utils::jwt::Claims;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpMessage, HttpRequest,
};
use futures::future::{ready, Ready};
#[derive(Clone)]
pub struct ServiceContext {
    pub user_id: i64,
    pub foundation_id: i64,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl From<Claims> for ServiceContext {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.user_id,
            foundation_id: claims.foundation_id,
            roles: claims.roles.clone(),
            permissions: claims.permissions.clone(),
        }
    }
}

// ← Tambahkan ini
impl FromRequest for ServiceContext {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let result = req
            .extensions()
            .get::<Claims>()
            .map(|claims| ServiceContext::from(claims.clone()))
            .ok_or_else(|| ErrorUnauthorized("Unauthorized: missing or invalid token"));

        ready(result)
    }
}

impl ServiceContext {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn foundation_id(&self) -> i64 {
        self.foundation_id
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin") || self.has_role("super_admin")
    }
}
