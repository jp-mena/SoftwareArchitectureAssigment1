use rocket::serde::json::Json;
use rocket::get;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{Template, context};
use crate::models::{ApiResponse, SimpleAuthorStats};
use crate::BookDb;

#[get("/simple-author-stats")]
pub async fn get_simple_author_stats(
    mut db: Connection<BookDb>,
) -> Result<Json<ApiResponse<Vec<SimpleAuthorStats>>>, rocket::http::Status> {
    match sqlx::query_as::<_, SimpleAuthorStats>("SELECT * FROM simple_author_stats")
        .fetch_all(&mut **db)
        .await
    {
        Ok(stats) => {
            Ok(Json(ApiResponse::success("Simple author statistics retrieved successfully", stats)))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[get("/admin/simple-author-stats")]
pub async fn admin_simple_author_stats(
    mut db: Connection<BookDb>,
) -> Result<Template, rocket::http::Status> {
    match sqlx::query_as::<_, SimpleAuthorStats>("SELECT * FROM simple_author_stats")
        .fetch_all(&mut **db)
        .await
    {
        Ok(stats) => {
            Ok(Template::render("admin_simple_stats", context! {
                stats: stats
            }))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
