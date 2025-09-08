use rocket::{get, Route, State};
use rocket::response::content;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct StaticConfig {
    pub serve_static: bool,
    pub static_path: String,
    pub upload_path: String,
}

impl StaticConfig {
    pub fn new() -> Self {
        Self {
            serve_static: std::env::var("SERVE_STATIC")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            static_path: std::env::var("STATIC_PATH")
                .unwrap_or_else(|_| "static".to_string()),
            upload_path: std::env::var("UPLOAD_PATH")
                .unwrap_or_else(|_| "uploads".to_string()),
        }
    }
}

pub fn get_static_routes() -> Vec<Route> {
    let config = StaticConfig::new();
    
    if config.serve_static {
        vec![
            get("/static/<path..>", serve_static_file),
            get("/uploads/<path..>", serve_upload_file),
        ]
    } else {
        vec![]
    }
}

#[get("/static/<path..>")]
async fn serve_static_file(path: std::path::PathBuf, config: &State<StaticConfig>) -> content::RawHtml<String> {
    if !config.serve_static {
        return content::RawHtml("Static files served by reverse proxy".to_string());
    }

    let static_path = Path::new(&config.static_path).join(path);
    
    if static_path.exists() && static_path.is_file() {
        match std::fs::read_to_string(&static_path) {
            Ok(content) => content::RawHtml(content),
            Err(_) => content::RawHtml("Error reading file".to_string()),
        }
    } else {
        content::RawHtml("File not found".to_string())
    }
}

#[get("/uploads/<path..>")]
async fn serve_upload_file(path: std::path::PathBuf, config: &State<StaticConfig>) -> content::RawHtml<String> {
    if !config.serve_static {
        return content::RawHtml("Upload files served by reverse proxy".to_string());
    }

    let upload_path = Path::new(&config.upload_path).join(path);
    
    if upload_path.exists() && upload_path.is_file() {
        match std::fs::read_to_string(&upload_path) {
            Ok(content) => content::RawHtml(content),
            Err(_) => content::RawHtml("Error reading file".to_string()),
        }
    } else {
        content::RawHtml("File not found".to_string())
    }
}

pub fn create_directories(config: &StaticConfig) -> Result<()> {
    std::fs::create_dir_all(&config.static_path)?;
    std::fs::create_dir_all(&config.upload_path)?;
    Ok(())
}