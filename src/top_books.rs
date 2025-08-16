use rocket::serde::json::Json;
use rocket::get;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{Template, context};
use crate::models::{ApiResponse};
use crate::BookDb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TopBook {
    pub id: i32,
    pub title: String,
    pub author_name: String,
    pub average_rating: f64,
    pub total_reviews: i64,
    pub best_review: Option<String>,
    pub best_review_score: Option<i32>,
    pub best_review_author: Option<String>,
    pub worst_review: Option<String>,
    pub worst_review_score: Option<i32>,
    pub worst_review_author: Option<String>,
}

#[get("/admin/top-books")]
pub async fn admin_top_books(
    mut db: Connection<BookDb>,
) -> Result<Template, rocket::http::Status> {
    match get_top_books_data(&mut db).await {
        Ok(books) => {
            Ok(Template::render("admin_top_books", context! {
                books: books
            }))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/admin/top-books/data")]
pub async fn admin_top_books_data(
    mut db: Connection<BookDb>,
) -> Result<Json<Vec<TopBook>>, rocket::http::Status> {
    match get_top_books_data(&mut db).await {
        Ok(books) => Ok(Json(books)),
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/top-books")]
pub async fn get_top_books(
    mut db: Connection<BookDb>,
) -> Result<Json<ApiResponse<Vec<TopBook>>>, rocket::http::Status> {
    match get_top_books_data(&mut db).await {
        Ok(books) => {
            Ok(Json(ApiResponse::success("Top 10 books retrieved successfully", books)))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

async fn get_top_books_data(db: &mut Connection<BookDb>) -> Result<Vec<TopBook>, sqlx::Error> {
    sqlx::query_as::<_, TopBook>(r#"
        WITH top_books AS (
            SELECT 
                b.id,
                b.name as title,
                a.name as author_name,
                COALESCE(AVG(r.score::float8), 0.0) as average_rating,
                COUNT(r.id)::bigint as total_reviews
            FROM books b
            JOIN authors a ON b.author_id = a.id
            LEFT JOIN reviews r ON b.id = r.book_id
            GROUP BY b.id, b.name, a.name
            HAVING COUNT(r.id) > 0
            ORDER BY average_rating DESC, total_reviews DESC
            LIMIT 10
        ),
        best_reviews AS (
            SELECT DISTINCT ON (r.book_id)
                r.book_id,
                r.review as best_review,
                r.score as best_review_score,
                r.reviewer_name as best_review_author
            FROM reviews r
            WHERE r.book_id IN (SELECT id FROM top_books)
            ORDER BY r.book_id, r.score DESC, r.number_of_upvotes DESC, r.created_at ASC
        ),
        worst_reviews AS (
            SELECT DISTINCT ON (r.book_id)
                r.book_id,
                r.review as worst_review,
                r.score as worst_review_score,
                r.reviewer_name as worst_review_author
            FROM reviews r
            WHERE r.book_id IN (SELECT id FROM top_books)
            ORDER BY r.book_id, r.score ASC, r.number_of_upvotes ASC, r.created_at ASC
        )
        SELECT 
            tb.id,
            tb.title,
            tb.author_name,
            tb.average_rating,
            tb.total_reviews,
            br.best_review,
            br.best_review_score,
            br.best_review_author,
            wr.worst_review,
            wr.worst_review_score,
            wr.worst_review_author
        FROM top_books tb
        LEFT JOIN best_reviews br ON tb.id = br.book_id
        LEFT JOIN worst_reviews wr ON tb.id = wr.book_id
        ORDER BY tb.average_rating DESC, tb.total_reviews DESC
    "#)
    .fetch_all(&mut ***db)
    .await
}
