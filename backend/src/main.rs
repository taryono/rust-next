// backend/src/main.rs
mod app_state;
mod config;
mod docs;
mod errors;
mod middleware;
mod modules;
mod routes;
mod utils;
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use config::database::Database;
use dotenv::dotenv;
mod states;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// ✨ Tambahkan import ini
use crate::app_state::AppState;
// mod macros;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::new(&database_url)
        .await
        .expect("Failed to connect to database");

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_addr = format!("{}:{}", host, port);
    // ✨ Gunakan macro yang auto-generated dari build.rs
    let openapi = generate_openapi!();
    // Check if Swagger should be enabled
    let enable_swagger = env::var("ENABLE_SWAGGER").unwrap_or("true".to_string()) == "true";

    let swagger_auth = env::var("SWAGGER_AUTH").unwrap_or("false".to_string()) == "true";
    // init service here

    log::info!("Starting server at http://{}", server_addr);
    // let auth_service = modules::auth::init_service(db.clone());
    // let academic_year_service = modules::academic_years::init_service(db.clone());
    // let permission_service = modules::permissions::init_service(db.clone());
    // let position_service = modules::positions::init_service(db.clone());
    // // ✨ Create AppState
    // let app_state = web::Data::new(AppState::new(
    //     academic_year_service,
    //     auth_service,
    //     permission_service,
    //     position_service,
    // ));
    let app_state = states::states::init_app(db.clone()).unwrap();
    if enable_swagger {
        if swagger_auth {
            log::info!(
                "Swagger UI available at http://{}/docs/swagger-ui/ (with authentication)",
                server_addr
            );
        } else {
            log::info!(
                "Swagger UI available at http://{}/swagger-ui/ (without authentication)",
                server_addr
            );
        }
    }

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(20)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // Frontend URL
            .allowed_origin("http://172.18.228.123:3000") // Frontend URL
            // .allowed_origin("https://yourfrontend.com")  // ← Production frontend
            // .allowed_origin(&format!("http://{}:3000", host))  // Dynamic
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials() // Jika perlu cookies
            .max_age(3600);

        let mut app = App::new()
            .wrap(Governor::new(&governor_conf))
            .app_data(web::Data::new(db.clone()))
            .app_data(app_state.clone()) // ← Tambahkan AppState
            .wrap(cors)
            .wrap(Logger::default())
            // Health check endpoint (GET - bisa di browser)
            .route(
                "/health",
                web::get().to(|| async {
                    HttpResponse::Ok().json(serde_json::json!({
                        "status": "ok",
                        "message": "Server is running"
                    }))
                }),
            )
            .configure(routes::configure);

        // Swagger UI configuration
        if enable_swagger {
            if swagger_auth {
                // Production mode: Swagger UI dengan authentication
                let openapi_for_json = openapi.clone();

                app = app
                    // OpenAPI JSON endpoint (tanpa auth - diperlukan oleh Swagger UI)
                    .route(
                        "/docs/openapi.json",
                        web::get().to(move || {
                            let api = openapi_for_json.clone();
                            async move { HttpResponse::Ok().json(api) }
                        }),
                    )
                    // Swagger UI dengan auth
                    .service(
                        web::scope("/docs")
                            .wrap(HttpAuthentication::basic(
                                middleware::swagger_auth::validator,
                            ))
                            .service(
                                SwaggerUi::new("/swagger-ui/{_:.*}")
                                    .url("/docs/openapi.json", openapi.clone()),
                            ),
                    );
            } else {
                // Development mode: Swagger UI tanpa authentication
                app = app.service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", openapi.clone()),
                );
            }
        }

        app
    })
    .bind(&server_addr)?
    .run()
    .await
}
