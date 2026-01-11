mod config;
mod controllers;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::database::Database;
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::auth_controller::register,
        controllers::auth_controller::login,
        controllers::auth_controller::refresh,
        controllers::user_controller::get_users,
        controllers::user_controller::get_user_by_id,
        controllers::user_controller::get_current_user,
        controllers::user_controller::update_current_user,
        controllers::user_controller::change_password,
        controllers::user_controller::delete_user,
    ),
    components(
        schemas(
            models::auth::RegisterRequest,
            models::auth::LoginRequest,
            models::auth::AuthResponse,
            models::auth::UserInfo,
            models::auth::RefreshTokenRequest,
            models::auth::RefreshTokenResponse,
            models::user::UserResponse,
            models::user::UserListResponse,
            models::user::UpdateUserRequest,
            models::user::ChangePasswordRequest,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db = Database::new(&database_url)
        .await
        .expect("Failed to connect to database");

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);

    log::info!("Starting server at http://{}", server_addr);
    log::info!("Swagger UI available at http://{}/swagger-ui/", server_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")  // Frontend URL
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
            actix_web::http::header::CONTENT_TYPE,
        ])
        .max_age(3600);

        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::auth_routes::configure)
            .configure(routes::user_routes::configure)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(&server_addr)?
    .run()
    .await
}