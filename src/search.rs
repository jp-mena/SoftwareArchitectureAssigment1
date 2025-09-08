use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct SearchService {
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchDocument {
    pub id: String,
    pub title: String,
    pub content: String,
    pub document_type: String,
    pub book_id: Option<i32>,
    pub author_id: Option<i32>,
    pub review_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<SearchHit>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchHit {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_source")]
    pub source: SearchDocument,
    #[serde(rename = "_score")]
    pub score: f64,
}

impl SearchService {
    pub fn new(_opensearch_url: Option<&str>) -> Result<Self> {
        // Por ahora, siempre deshabilitado para simplificar
        Ok(Self {
            enabled: false,
        })
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub async fn index_book(&self, _book: &crate::models::Book, _author_name: &str) -> Result<()> {
        Ok(())
    }

    pub async fn search(&self, _query: &str) -> Result<SearchResult> {
        Ok(SearchResult {
            hits: vec![],
            total: 0,
        })
    }
}

// Funciones para las rutas de administración
use rocket::{get, Route};
use rocket_dyn_templates::{Template, context};

pub fn get_search_routes() -> Vec<Route> {
    vec![
        get("/admin/search", admin_search),
        get("/admin/search/data", admin_search_data),
    ]
}

#[get("/admin/search")]
fn admin_search() -> Template {
    Template::render("admin_search", context! {})
}

#[get("/admin/search/data")]
async fn admin_search_data() -> rocket::serde::json::Json<SearchResult> {
    // Por ahora devolvemos un resultado vacío
    rocket::serde::json::Json(SearchResult {
        hits: vec![],
        total: 0,
    })
}

#[get("/api/search/books?<_q>")]
pub async fn api_search_books(_q: Option<String>) -> rocket::serde::json::Json<SearchResult> {
    // Por ahora devolvemos un resultado vacío
    rocket::serde::json::Json(SearchResult {
        hits: vec![],
        total: 0,
    })
}