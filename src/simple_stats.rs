use rocket::serde::json::Json;
use rocket::get;
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{Template, context};
use crate::models::{ApiResponse, SimpleAuthorStats};
use crate::BookDb;

#[get("/simple-author-stats?<sort_by>&<sort_order>&<filter_name>&<min_books>&<min_rating>&<min_sales>")]
pub async fn get_simple_author_stats(
    mut db: Connection<BookDb>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    filter_name: Option<String>,
    min_books: Option<i64>,
    min_rating: Option<f64>,
    min_sales: Option<i64>,
) -> Result<Json<ApiResponse<Vec<SimpleAuthorStats>>>, rocket::http::Status> {
    match sqlx::query_as::<_, SimpleAuthorStats>("SELECT * FROM simple_author_stats")
        .fetch_all(&mut **db)
        .await
    {
        Ok(mut stats) => {
            // Aplicar filtros
            if let Some(name) = filter_name {
                stats.retain(|stat| {
                    stat.author_name.to_lowercase().contains(&name.to_lowercase())
                });
            }
            
            if let Some(min_b) = min_books {
                stats.retain(|stat| stat.total_books >= min_b);
            }
            
            if let Some(min_r) = min_rating {
                stats.retain(|stat| stat.average_rating >= min_r);
            }
            
            if let Some(min_s) = min_sales {
                stats.retain(|stat| stat.total_sales >= min_s);
            }

            // Aplicar ordenamiento
            match sort_by.as_deref() {
                Some("books") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.total_books.cmp(&a.total_books));
                    } else {
                        stats.sort_by(|a, b| a.total_books.cmp(&b.total_books));
                    }
                },
                Some("rating") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| {
                            b.average_rating
                                .partial_cmp(&a.average_rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    } else {
                        stats.sort_by(|a, b| {
                            a.average_rating
                                .partial_cmp(&b.average_rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                },
                Some("sales") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.total_sales.cmp(&a.total_sales));
                    } else {
                        stats.sort_by(|a, b| a.total_sales.cmp(&b.total_sales));
                    }
                },
                Some("name") | _ => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.author_name.cmp(&a.author_name));
                    } else {
                        stats.sort_by(|a, b| a.author_name.cmp(&b.author_name));
                    }
                }
            }

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

#[get("/admin/simple-stats/data?<sort_by>&<sort_order>&<filter_name>&<min_books>&<min_rating>&<min_sales>")]
pub async fn admin_simple_stats_data(
    mut db: Connection<BookDb>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    filter_name: Option<String>,
    min_books: Option<i64>,
    min_rating: Option<f64>,
    min_sales: Option<i64>,
) -> Result<Json<Vec<SimpleAuthorStats>>, rocket::http::Status> {
    match sqlx::query_as::<_, SimpleAuthorStats>("SELECT * FROM simple_author_stats")
        .fetch_all(&mut **db)
        .await
    {
        Ok(mut stats) => {
            // Aplicar filtros
            if let Some(name) = filter_name {
                stats.retain(|stat| {
                    stat.author_name.to_lowercase().contains(&name.to_lowercase())
                });
            }
            
            if let Some(min_b) = min_books {
                stats.retain(|stat| stat.total_books >= min_b);
            }
            
            if let Some(min_r) = min_rating {
                stats.retain(|stat| stat.average_rating >= min_r);
            }
            
            if let Some(min_s) = min_sales {
                stats.retain(|stat| stat.total_sales >= min_s);
            }

            // Aplicar ordenamiento
            match sort_by.as_deref() {
                Some("books") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.total_books.cmp(&a.total_books));
                    } else {
                        stats.sort_by(|a, b| a.total_books.cmp(&b.total_books));
                    }
                },
                Some("rating") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| {
                            b.average_rating
                                .partial_cmp(&a.average_rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    } else {
                        stats.sort_by(|a, b| {
                            a.average_rating
                                .partial_cmp(&b.average_rating)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                    }
                },
                Some("sales") => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.total_sales.cmp(&a.total_sales));
                    } else {
                        stats.sort_by(|a, b| a.total_sales.cmp(&b.total_sales));
                    }
                },
                Some("name") | _ => {
                    if sort_order.as_deref() == Some("desc") {
                        stats.sort_by(|a, b| b.author_name.cmp(&a.author_name));
                    } else {
                        stats.sort_by(|a, b| a.author_name.cmp(&b.author_name));
                    }
                }
            }

            Ok(Json(stats))
        }
        Err(e) => {
            println!("Database error: {}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}
