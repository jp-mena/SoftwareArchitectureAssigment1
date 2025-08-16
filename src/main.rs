#[macro_use] extern crate rocket;

mod models;
mod authors;
mod books;
mod reviews;
mod sales;
mod simple_stats;
mod top_books;
mod top_sales;
mod search;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket_db_pools::{Database, Connection};
use sqlx;

#[derive(Database)]
#[database("book_db")]
pub struct BookDb(sqlx::PgPool);

#[get("/health/db")]
async fn health_db(mut db: Connection<BookDb>) -> Result<&'static str, Status> {
    // Test basic connection
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&mut **db)
        .await
    {
        Ok(_n) => {
            // Test if our tables exist
            match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN ('authors', 'books', 'reviews', 'sales')")
                .fetch_one(&mut **db)
                .await
            {
                Ok(count) => {
                    if count == 4 {
                        Ok("DB OK - All tables created ✅")
                    } else {
                        Ok("DB OK - But missing tables (run migrations) ⚠️")
                    }
                },
                Err(_) => Ok("DB OK - But schema check failed ⚠️")
            }
        },
        Err(e) => {
            eprintln!("DB health error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

// === RUTAS PARA ADMIN WEB INTERFACE ===

#[get("/admin")]
fn admin_home() -> Template {
    Template::render("admin_home", context! {})
}

#[get("/admin/authors")]
fn admin_authors_list() -> Template {
    Template::render("authors_list", context! {})
}

#[get("/admin/authors/new")]
fn admin_authors_new() -> Template {
    Template::render("authors_new", context! {})
}

// === BOOKS ADMIN ROUTES ===

#[get("/admin/books")]
fn admin_books_list() -> Template {
    Template::render("books_list", context! {})
}

#[get("/admin/books/new")]
fn admin_books_new() -> Template {
    Template::render("books_new", context! {})
}

// === REVIEWS ADMIN ROUTES ===

#[get("/admin/reviews")]
fn admin_reviews_list() -> Template {
    Template::render("reviews_list", context! {})
}

#[get("/admin/reviews/new")]
fn admin_reviews_new() -> Template {
    Template::render("reviews_new", context! {})
}

// === SALES ADMIN ROUTES ===

#[get("/admin/sales")]
fn admin_sales_list() -> Template {
    Template::render("sales_list", context! {})
}

#[get("/admin/sales/new")]
fn admin_sales_new() -> Template {
    Template::render("sales_new", context! {})
}

// === TOP BOOKS ADMIN ROUTES ===

#[get("/admin/top-books")]
fn admin_top_books_route() -> Template {
    Template::render("admin_top_books", context! {})
}

// Redirección desde raíz a admin
#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(admin_home))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(<BookDb as Database>::init())
        .attach(Template::fairing())
        .mount("/", routes![
            index, 
            health_db, 
            admin_home, 
            admin_authors_list, 
            admin_authors_new,
            admin_books_list,
            admin_books_new,
            admin_reviews_list,
            admin_reviews_new,
            admin_sales_list,
            admin_sales_new,
            simple_stats::admin_simple_author_stats,
            simple_stats::admin_simple_stats_data,
            top_books::admin_top_books,
            top_books::admin_top_books_data,
            top_sales::admin_top_sales,
            top_sales::admin_top_sales_data,
            search::admin_search,
            search::admin_search_data
        ])
        .mount("/api", authors::routes())
        .mount("/api", books::routes())
        .mount("/api", reviews::routes())
        .mount("/api", sales::routes())
        .mount("/api", routes![simple_stats::get_simple_author_stats])
        .mount("/api", routes![top_books::get_top_books])
        .mount("/api", routes![top_sales::get_top_sales])
        .mount("/api", routes![search::api_search_books])
}
