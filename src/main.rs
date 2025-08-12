#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx;

#[derive(Database)]
#[database("book_db")]
struct BookDb(sqlx::PgPool);

#[get("/")]
fn index() -> &'static str {
    "¡Bienvenido a Book Reviews API! 📚
    
    Rutas disponibles:
    - GET / (esta página)
    - GET /health/db (verificar conexión a base de datos)
    
    El servidor está funcionando correctamente 🚀
    
    💡 Para aplicar migraciones usa: sqlx migrate run"
}

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(BookDb::init())
        .mount("/", routes![index, health_db])
}
