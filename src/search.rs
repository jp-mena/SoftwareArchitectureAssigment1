use rocket::serde::json::Json;
use rocket::get;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{Template, context};
use crate::models::{ApiResponse};
use crate::BookDb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: i32,
    pub title: String,
    pub author_name: String,
    pub summary: Option<String>,
    pub date_of_publication: Option<chrono::NaiveDate>,
    pub total_sales: i64,
    pub average_rating: f64,
    pub total_reviews: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedSearchResults {
    pub results: Vec<SearchResult>,
    pub total_count: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[get("/admin/search")]
pub async fn admin_search() -> Template {
    Template::render("admin_search", context! {})
}

#[get("/admin/search/data?<query>&<page>&<per_page>")]
pub async fn admin_search_data(
    mut db: Connection<BookDb>,
    query: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<Json<PaginatedSearchResults>, rocket::http::Status> {
    let search_query = query.unwrap_or_default();
    let current_page = page.unwrap_or(1).max(1);
    let items_per_page = per_page.unwrap_or(10).min(50).max(1); // Límite entre 1 y 50
    let offset = (current_page - 1) * items_per_page;

    if search_query.trim().is_empty() {
        return Ok(Json(PaginatedSearchResults {
            results: vec![],
            total_count: 0,
            page: current_page,
            per_page: items_per_page,
            total_pages: 0,
            has_next: false,
            has_prev: false,
        }));
    }

    match search_books(&mut db, &search_query, offset, items_per_page).await {
        Ok((results, total_count)) => {
            let total_pages = ((total_count as f64) / (items_per_page as f64)).ceil() as i32;
            
            Ok(Json(PaginatedSearchResults {
                results,
                total_count,
                page: current_page,
                per_page: items_per_page,
                total_pages,
                has_next: current_page < total_pages,
                has_prev: current_page > 1,
            }))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/search-books?<query>&<page>&<per_page>")]
pub async fn api_search_books(
    mut db: Connection<BookDb>,
    query: Option<String>,
    page: Option<i32>,
    per_page: Option<i32>,
) -> Result<Json<ApiResponse<PaginatedSearchResults>>, rocket::http::Status> {
    let search_query = query.unwrap_or_default();
    let current_page = page.unwrap_or(1).max(1);
    let items_per_page = per_page.unwrap_or(10).min(50).max(1);
    let offset = (current_page - 1) * items_per_page;

    if search_query.trim().is_empty() {
        let empty_results = PaginatedSearchResults {
            results: vec![],
            total_count: 0,
            page: current_page,
            per_page: items_per_page,
            total_pages: 0,
            has_next: false,
            has_prev: false,
        };
        
        return Ok(Json(ApiResponse::success("Empty search query", empty_results)));
    }

    match search_books(&mut db, &search_query, offset, items_per_page).await {
        Ok((results, total_count)) => {
            let total_pages = ((total_count as f64) / (items_per_page as f64)).ceil() as i32;
            
            let paginated_results = PaginatedSearchResults {
                results,
                total_count,
                page: current_page,
                per_page: items_per_page,
                total_pages,
                has_next: current_page < total_pages,
                has_prev: current_page > 1,
            };

            Ok(Json(ApiResponse::success("Books search completed successfully", paginated_results)))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

async fn search_books(
    db: &mut Connection<BookDb>,
    query: &str,
    offset: i32,
    limit: i32,
) -> Result<(Vec<SearchResult>, i64), sqlx::Error> {
    // Preparar las palabras de búsqueda
    let search_words: Vec<String> = query
        .split_whitespace()
        .map(|word| format!("%{}%", word.to_lowercase()))
        .collect();

    if search_words.is_empty() {
        return Ok((vec![], 0));
    }

    // Construir condiciones WHERE dinámicamente con parámetros numerados
    let mut where_conditions = Vec::new();
    for i in 0..search_words.len() {
        where_conditions.push(format!("LOWER(b.summary) LIKE ${}", i + 1));
    }
    let where_clause = where_conditions.join(" OR ");

    // Consulta para contar el total
    let count_sql = format!(r#"
        SELECT COUNT(DISTINCT b.id)::bigint
        FROM books b
        WHERE b.summary IS NOT NULL AND ({})
    "#, where_clause);

    // Consulta para obtener resultados con paginación
    let search_sql = format!(r#"
        SELECT 
            b.id,
            b.name as title,
            a.name as author_name,
            b.summary,
            b.date_of_publication,
            COALESCE(SUM(s.sales), 0)::bigint as total_sales,
            COALESCE(AVG(r.score::float8), 0.0) as average_rating,
            COUNT(r.id)::bigint as total_reviews
        FROM books b
        JOIN authors a ON b.author_id = a.id
        LEFT JOIN sales s ON b.id = s.book_id
        LEFT JOIN reviews r ON b.id = r.book_id
        WHERE b.summary IS NOT NULL AND ({})
        GROUP BY b.id, b.name, a.name, b.summary, b.date_of_publication
        ORDER BY average_rating DESC, total_sales DESC, b.name ASC
        LIMIT ${} OFFSET ${}
    "#, where_clause, search_words.len() + 1, search_words.len() + 2);

    // Ejecutar consulta de conteo
    let mut count_query = sqlx::query_scalar::<_, i64>(&count_sql);
    for word in &search_words {
        count_query = count_query.bind(word);
    }
    let total_count = count_query.fetch_one(&mut ***db).await?;

    // Ejecutar consulta de resultados
    let mut results_query = sqlx::query_as::<_, SearchResult>(&search_sql);
    for word in &search_words {
        results_query = results_query.bind(word);
    }
    results_query = results_query.bind(limit);
    results_query = results_query.bind(offset);
    
    let results = results_query.fetch_all(&mut ***db).await?;

    Ok((results, total_count))
}
