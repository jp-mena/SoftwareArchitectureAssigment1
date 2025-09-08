use rocket::{post, Route, Data, State, response::status::Created};
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use std::path::Path;
use anyhow::Result;
use crate::models::ApiResponse;

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub file_path: String,
    pub file_name: String,
    pub file_size: u64,
}

pub fn get_upload_routes() -> Vec<Route> {
    vec![
        post("/upload/image", upload_image),
    ]
}

#[post("/upload/image", data = "<_data>")]
async fn upload_image(
    _data: Data<'_>,
    _config: &State<crate::static_files::StaticConfig>,
) -> Result<Created<Json<ApiResponse<UploadResponse>>>, rocket::response::status::BadRequest<Json<ApiResponse<()>>>> {
    
    // Por ahora, solo devolvemos un mensaje de que la funcionalidad está en desarrollo
    let response = UploadResponse {
        file_path: "uploads/placeholder.jpg".to_string(),
        file_name: "placeholder.jpg".to_string(),
        file_size: 0,
    };

    Ok(Created::new("/api/upload/image").body(Json(
        ApiResponse::success("Funcionalidad de carga de archivos en desarrollo", response)
    )))
}

// Función auxiliar para eliminar archivos
pub async fn delete_uploaded_file(config: &crate::static_files::StaticConfig, file_path: &str) -> Result<()> {
    let full_path = Path::new(&config.upload_path).join(file_path);
    
    if full_path.exists() {
        std::fs::remove_file(&full_path)?;
    }
    
    Ok(())
}

// Función auxiliar para validar si un archivo existe
pub fn file_exists(config: &crate::static_files::StaticConfig, file_path: &str) -> bool {
    let full_path = Path::new(&config.upload_path).join(file_path);
    full_path.exists() && full_path.is_file()
}