// api/src/config/app.rs
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub upload_path: String,
    pub max_upload_size: usize, // dalam bytes
    pub allowed_image_extensions: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            upload_path: env::var("UPLOAD_PATH").unwrap_or_else(|_| "uploads".to_string()),

            max_upload_size: env::var("MAX_UPLOAD_SIZE_MB")
                .unwrap_or_else(|_| "5".to_string())
                .parse::<usize>()
                .unwrap_or(5)
                * 1024
                * 1024, // convert MB → bytes

            allowed_image_extensions: env::var("ALLOWED_IMAGE_EXTENSIONS")
                .unwrap_or_else(|_| "jpg,jpeg,png,webp".to_string())
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .collect(),
        }
    }

    /// Helper untuk generate path upload per modul
    /// Contoh: app_config.upload_dir_for("users") → "uploads/users"
    pub fn upload_dir_for(&self, module: &str) -> String {
        format!("{}/{}", self.upload_path, module)
    }
}
