use crate::utils::jwt::verify_token;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, Result,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

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
        let auth_header = req.headers().get("Authorization");

        let token = match auth_header {
            Some(header) => match header.to_str() {
                Ok(header_str) => {
                    if header_str.starts_with("Bearer ") {
                        Some(header_str[7..].to_string())
                    } else {
                        None
                    }
                }
                Err(_) => None,
            },
            None => None,
        };

        match token {
            Some(token) => match verify_token(&token) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    let fut = self.service.call(req);
                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(_) => Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                }),
            },
            None => Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized("No token provided"))
            }),
        }
    }
}