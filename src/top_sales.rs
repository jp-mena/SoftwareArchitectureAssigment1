use rocket::serde::json::Json;
use rocket::get;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{Template, context};
use crate::models::{ApiResponse};
use crate::BookDb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopSalesBook {
    pub id: i32,
    pub title: String,
    pub author_name: String,
    pub total_sales: i64,
    pub author_total_sales: i64,
    pub publication_year: Option<i32>,
    pub was_top5_in_publication_year: bool,
}

#[get("/admin/top-sales")]
pub async fn admin_top_sales(
    mut db: Connection<BookDb>,
) -> Result<Template, rocket::http::Status> {
    match get_top_sales_data(&mut db).await {
        Ok(books) => {
            Ok(Template::render("admin_top_sales", context! {
                books: books
            }))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/admin/top-sales/data")]
pub async fn admin_top_sales_data(
    mut db: Connection<BookDb>,
) -> Result<Json<Vec<TopSalesBook>>, rocket::http::Status> {
    match get_top_sales_data(&mut db).await {
        Ok(books) => Ok(Json(books)),
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/top-sales")]
pub async fn get_top_sales(
    mut db: Connection<BookDb>,
) -> Result<Json<ApiResponse<Vec<TopSalesBook>>>, rocket::http::Status> {
    match get_top_sales_data(&mut db).await {
        Ok(books) => {
            Ok(Json(ApiResponse::success("Top 50 best-selling books retrieved successfully", books)))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

async fn get_top_sales_data(db: &mut Connection<BookDb>) -> Result<Vec<TopSalesBook>, sqlx::Error> {
    sqlx::query_as::<_, TopSalesBook>(r#"
        WITH book_sales AS (
            -- Calcular ventas totales por libro
            SELECT 
                b.id,
                b.name as title,
                a.name as author_name,
                b.author_id,
                EXTRACT(YEAR FROM b.date_of_publication) as publication_year,
                COALESCE(SUM(s.sales), 0)::bigint as total_sales
            FROM books b
            JOIN authors a ON b.author_id = a.id
            LEFT JOIN sales s ON b.id = s.book_id
            GROUP BY b.id, b.name, a.name, b.author_id, b.date_of_publication
        ),
        author_total_sales AS (
            -- Calcular ventas totales por autor
            SELECT 
                a.id as author_id,
                COALESCE(SUM(s.sales), 0)::bigint as author_total_sales
            FROM authors a
            JOIN books b ON a.id = b.author_id
            LEFT JOIN sales s ON b.id = s.book_id
            GROUP BY a.id
        ),
        yearly_rankings AS (
            -- Determinar ranking de cada libro en su año de publicación
            SELECT 
                bs.id,
                bs.publication_year,
                bs.total_sales,
                ROW_NUMBER() OVER (
                    PARTITION BY bs.publication_year 
                    ORDER BY bs.total_sales DESC
                ) as yearly_rank
            FROM book_sales bs
            WHERE bs.publication_year IS NOT NULL
        )
        SELECT 
            bs.id,
            bs.title,
            bs.author_name,
            bs.total_sales,
            ats.author_total_sales,
            bs.publication_year::integer,
            COALESCE(yr.yearly_rank <= 5, false) as was_top5_in_publication_year
        FROM book_sales bs
        LEFT JOIN author_total_sales ats ON bs.author_id = ats.author_id
        LEFT JOIN yearly_rankings yr ON bs.id = yr.id
        ORDER BY bs.total_sales DESC, bs.title ASC
        LIMIT 50
    "#)
    .fetch_all(&mut ***db)
    .await
}
