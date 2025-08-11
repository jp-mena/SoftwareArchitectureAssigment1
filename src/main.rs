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
    
    El servidor está funcionando correctamente 🚀"
}

#[get("/health/db")]
async fn health_db(mut db: Connection<BookDb>) -> Result<&'static str, Status> {
    // OJO: doble deref para llegar a PgConnection
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&mut **db)
        .await
    {
        Ok(_n) => Ok("DB OK"),
        Err(e) => {
            eprintln!("DB health error: {e:?}"); // verás el error en la terminal
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
