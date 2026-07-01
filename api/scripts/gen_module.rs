use clap::{ArgAction, Parser};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about = "Module Generator CLI (Project-aware)", long_about = None)]
struct Args {
    /// Module name (snake_case), e.g. academic_years
    pub module: String,

    /// Force overwrite if module exists
    #[arg(long, action = ArgAction::SetTrue)]
    pub force: bool,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let module = args.module.to_lowercase();
    let module_pascal = to_pascal_case(&module);
    let module_entity = module.clone(); // entity::module
    let base_path = Path::new("src/modules").join(&module);

    if base_path.exists() && !args.force {
        error!(
            "Module '{}' already exists. Use --force to overwrite.",
            module
        );
        std::process::exit(1);
    }

    if let Err(e) = fs::create_dir_all(&base_path) {
        error!("Failed to create module directory: {}", e);
        std::process::exit(1);
    }

    info!("Generating module: {}", module);

    create_file(
        &base_path,
        "handler.rs",
        handler_rs(&module, &module_pascal),
    );
    create_file(&base_path, "mod.rs", mod_rs());
    create_file(&base_path, "routes.rs", routes_rs(&module));
    create_file(
        &base_path,
        "service.rs",
        service_rs(&module, &module_pascal),
    );
    create_file(&base_path, "dto.rs", dto_rs(&module_pascal));
    create_file(&base_path, "docs.rs", docs_rs(&module_pascal));
    create_file(
        &base_path,
        "repository.rs",
        repository_rs(&module, &module_pascal),
    );

    register_module(&module);

    info!("✅ Module '{}' generated successfully", module);
}

fn create_file(base: &Path, name: &str, content: String) {
    let path = base.join(name);

    if path.exists() {
        info!("Skipping existing file: {:?}", path);
        return;
    }

    if let Err(e) = fs::write(&path, content) {
        error!("Failed to write {:?}: {}", path, e);
    }
}

fn to_pascal_case(input: &str) -> String {
    input
        .split('_')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn register_module(module: &str) {
    let mod_file = PathBuf::from("src/modules/mod.rs");

    let line = format!("pub mod {};\n", module);

    let mut content = fs::read_to_string(&mod_file).unwrap_or_default();

    if !content.contains(&line) {
        content.push_str(&line);
        if let Err(e) = fs::write(&mod_file, content) {
            error!("Failed to update mod.rs: {}", e);
        } else {
            info!("Registered module '{}' in mod.rs", module);
        }
    }
}

// ========================= TEMPLATES (PROJECT-AWARE) =========================

fn repository_rs(module: &str, name: &str) -> String {
    format!(
        r#"use crate::config::database::Database;
        use crate::errors::AppError;
        use crate::utils::pagination::PaginationParams;
        use entity::{0}::{{self, Entity as {1}}};
        use sea_orm::{{
            ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
        }};

        #[derive(Clone)]
        pub struct {1}Repository {{
            db: Database,
        }}

        impl {1}Repository {{
            pub fn new(db: Database) -> Self {{
                Self {{ db }}
            }}

            pub fn conn(&self) -> &sea_orm::DatabaseConnection {{
                self.db.get_connection()
            }}

            pub async fn create(&self, active_model: {0}::ActiveModel) -> Result<{0}::Model, AppError> {{
                active_model.insert(self.conn()).await.map_err(|e| AppError::DatabaseError(e.to_string()))
            }}

            pub async fn find_by_id(&self, id: i64) -> Result<Option<{0}::Model>, AppError> {{
                {1}::find_by_id(id).one(self.conn()).await.map_err(|e| AppError::DatabaseError(e.to_string()))
            }}

            pub async fn find_all(&self, params: &PaginationParams) -> Result<(Vec<{0}::Model>, u64), AppError> {{
                let query = {1}::find();
                let paginator = query.paginate(self.conn(), params.per_page() as u64);

                let total = paginator.num_items().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
                let items = paginator.fetch_page((params.page() - 1) as u64).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;

                Ok((items, total.try_into().unwrap_or(0)))
            }}

            pub async fn update(&self, id: i64, mut active_model: {0}::ActiveModel) -> Result<{0}::Model, AppError> {{
                active_model.id = Set(id);
                active_model.update(self.conn()).await.map_err(|e| AppError::DatabaseError(e.to_string()))
            }}

            pub async fn delete(&self, id: i64) -> Result<(), AppError> {{
                {1}::delete_by_id(id).exec(self.conn()).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
                Ok(())
            }}
        }}
        "#,
        module, name
    )
}

fn service_rs(module: &str, name: &str) -> String {
    format!(
        r#"use super::dto::{{{1}Response, Create{1}Request, Update{1}Request}};
        use super::repository::{1}Repository;
        use crate::errors::AppError;
        use crate::utils::pagination::{{PaginatedResponse, PaginationParams}};
        use entity::{0};
        use sea_orm::Set;
        use validator::Validate;

        #[derive(Clone)]
        pub struct {1}Service {{
            repository: {1}Repository,
        }}

        impl {1}Service {{
            pub fn new(repository: {1}Repository) -> Self {{
                Self {{ repository }}
            }}

            pub async fn create(&self, request: Create{1}Request) -> Result<{1}Response, AppError> {{
                request.validate().map_err(|e| AppError::validation(e.to_string()))?;

                let active_model = {0}::ActiveModel {{
                    ..Default::default()
                }};

                let created = self.repository.create(active_model).await?;
                Ok({1}Response::from(created))
            }}

            pub async fn get_by_id(&self, id: i64) -> Result<{1}Response, AppError> {{
                let item = self.repository.find_by_id(id).await?
                    .ok_or_else(|| AppError::not_found("Data not found".to_string()))?;

                Ok({1}Response::from(item))
            }}

            pub async fn get_all(&self, params: PaginationParams) -> Result<PaginatedResponse<{1}Response>, AppError> {{
                params.validate().map_err(|e| AppError::validation(e.to_string()))?;

                let (items, total) = self.repository.find_all(&params).await?;

                let responses: Vec<{1}Response> = items.into_iter().map({1}Response::from).collect();

                Ok(PaginatedResponse::new(responses, total, params.page(), params.per_page()))
            }}

            pub async fn update(&self, id: i64, request: Update{1}Request) -> Result<{1}Response, AppError> {{
                request.validate().map_err(|e| AppError::validation(e.to_string()))?;

                let active_model = {0}::ActiveModel {{
                    ..Default::default()
                }};

                let updated = self.repository.update(id, active_model).await?;
                Ok({1}Response::from(updated))
            }}

            pub async fn delete(&self, id: i64) -> Result<(), AppError> {{
                self.repository.delete(id).await
            }}
        }}
        "#,
        module, name
    )
}

fn handler_rs(module: &str, name: &str) -> String {
    format!(
        r#"use super::dto::{{{1}Response, Create{1}Request, Update{1}Request}};
        use crate::app_state::AppState;
        use crate::errors::AppError;
        use crate::utils::pagination::PaginationParams;
        use actix_web::{{web, HttpResponse}};

        pub async fn create(app_state: web::Data<AppState>, request: web::Json<Create{1}Request>) -> Result<HttpResponse, AppError> {{
            let result = app_state.{0}_service.create(request.into_inner()).await?;
            Ok(HttpResponse::Created().json(result))
        }}

        pub async fn get_by_id(app_state: web::Data<AppState>, id: web::Path<i64>) -> Result<HttpResponse, AppError> {{
            let result = app_state.{0}_service.get_by_id(id.into_inner()).await?;
            Ok(HttpResponse::Ok().json(result))
        }}

        pub async fn get_all(app_state: web::Data<AppState>, query: web::Query<PaginationParams>) -> Result<HttpResponse, AppError> {{
            let result = app_state.{0}_service.get_all(query.into_inner()).await?;
            Ok(HttpResponse::Ok().json(result))
        }}

        pub async fn update(app_state: web::Data<AppState>, id: web::Path<i64>, request: web::Json<Update{1}Request>) -> Result<HttpResponse, AppError> {{
            let result = app_state.{0}_service.update(id.into_inner(), request.into_inner()).await?;
            Ok(HttpResponse::Ok().json(result))
        }}

        pub async fn delete(app_state: web::Data<AppState>, id: web::Path<i64>) -> Result<HttpResponse, AppError> {{
            app_state.{0}_service.delete(id.into_inner()).await?;
            Ok(HttpResponse::NoContent().finish())
        }}
        "#,
        module, name
    )
}

fn routes_rs(module: &str) -> String {
    format!(
        r#"use actix_web::web;
        use super::handler;

        pub fn configure(cfg: &mut web::ServiceConfig) {{
            cfg.service(
                web::scope("/{0}")
                    .route("", web::get().to(handler::get_all))
                    .route("", web::post().to(handler::create))
                    .route("/{{id}}", web::get().to(handler::get_by_id))
                    .route("/{{id}}", web::put().to(handler::update))
                    .route("/{{id}}", web::delete().to(handler::delete))
            );
        }}
        "#,
        module
    )
}

fn dto_rs(name: &str) -> String {
    format!(
        r#"use serde::{{Deserialize, Serialize}};
        use utoipa::ToSchema;
        use validator::Validate;

        #[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
        pub struct Create{0}Request {{}}

        #[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
        pub struct Update{0}Request {{}}

        #[derive(Debug, Serialize, Deserialize, ToSchema)]
        pub struct {0}Response {{}}
        "#,
        name
    )
}

fn docs_rs(name: &str) -> String {
    format!(
        r#"use utoipa::OpenApi;

        #[derive(OpenApi)]
        #[openapi(paths(), components(schemas()))]
        pub struct {0}ApiDoc;
        "#,
        name
    )
}

fn mod_rs() -> String {
    r#"pub mod handler;
    pub mod routes;
    pub mod service;
    pub mod dto;
    pub mod docs;
    pub mod repository;
    "#
    .to_string()
}
