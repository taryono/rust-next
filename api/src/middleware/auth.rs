// api/src/middleware/auth.rs
use crate::utils::jwt::verify_token; // ← import Claims
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, Result,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

// ← Hapus AuthContext, tidak diperlukan lagi

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_string());

        match token {
            Some(token) => {
                match verify_token(&token) {
                    Ok(claims) => {
                        log::debug!(
                            "🔐 Auth successful - user_id: {}, foundation_id: {}",
                            claims.user_id,
                            claims.foundation_id
                        );

                        // ← Insert Claims langsung ke extensions
                        req.extensions_mut().insert(claims);

                        let fut = self.service.call(req);
                        Box::pin(async move { fut.await })
                    }
                    Err(_) => Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                    }),
                }
            }
            None => {
                Box::pin(
                    async move { Err(actix_web::error::ErrorUnauthorized("No token provided")) },
                )
            }
        }
    }
}
