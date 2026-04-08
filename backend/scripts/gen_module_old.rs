// backend/scripts/gen_module.rs
use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin gen_module <module_name>");
        std::process::exit(1);
    }

    let module = args[1].to_lowercase();
    let base_path = Path::new("src/modules").join(&module);

    if base_path.exists() {
        eprintln!("Module `{}` already exists", module);
        return;
    }

    fs::create_dir_all(&base_path).expect("Failed to create module directory");

    // ✅ Perbaikan: ganti `base` dengan `base_path`
    create_file(&base_path, format!("{}.rs", module), handler_rs(&module));
    create_file(&base_path, "mod.rs", mod_rs(&module));
    create_file(&base_path, "routes.rs", routes_rs(&module));
    create_file(&base_path, "service.rs", service_rs(&module));
    create_file(&base_path, "dto.rs", dto_rs(&module));
    create_file(&base_path, "docs.rs", docs_rs(&module));
    create_file(&base_path, "repository.rs", repository_rs(&module));

    println!("✅ Module `{}` generated successfully", module);
}

fn create_file<P: AsRef<Path>>(base: P, name: impl AsRef<Path>, content: String) {
    let path = base.as_ref().join(name);
    // fs::write(path, content).expect("Failed to write file");
    if let Err(e) = fs::write(&path, content) {
        eprintln!("Failed to write {:?}: {}", path, e);
    }
}

// ⚠️ Tambahkan stub functions jika belum ada
fn handler_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/handler.rs
use actix_web::{{web, HttpResponse, Result}};

pub async fn health_check() -> Result<HttpResponse> {{
    Ok(HttpResponse::Ok().json("OK"))
}}
"#,
        module
    )
}

fn mod_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/mod.rs
pub mod routes;
pub mod service;
pub mod dto;
pub mod docs;
pub mod repository;
"#,
        module
    )
}

fn routes_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/routes.rs
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {{
    cfg.service(
        web::scope("/{}"),
    );
}}
"#,
        module, module
    )
}

fn service_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/service.rs
use crate::errors::AppError;

pub struct {}Service {{
    // Add your dependencies here
}}

impl {}Service {{
    pub fn new() -> Self {{
        Self {{}}
    }}
}}
"#,
        module,
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>(),
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>()
    )
}

fn dto_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/dto.rs
use serde::{{Deserialize, Serialize}};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct {}Request {{
    // Add your fields here
}}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct {}Response {{
    // Add your fields here
}}
"#,
        module,
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>(),
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>()
    )
}

fn docs_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/docs.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(),
    components(schemas())
)]
pub struct {}ApiDoc;
"#,
        module,
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>()
    )
}

fn repository_rs(module: &str) -> String {
    format!(
        r#"// backend/src/modules/{}/repository.rs
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct {}Repository {{
    db: DatabaseConnection,
}}

impl {}Repository {{
    pub fn new(db: DatabaseConnection) -> Self {{
        Self {{ db }}
    }}
}}
"#,
        module,
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>(),
        module
            .split('_')
            .map(|s| {
                {
                    let mut c = s.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                }
            })
            .collect::<String>()
    )
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
