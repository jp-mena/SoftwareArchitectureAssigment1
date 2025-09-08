#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

// Versión simplificada sin base de datos para probar que funciona
#[get("/health")]
fn health() -> &'static str {
    "OK - Aplicación funcionando ✅"
}

#[get("/")]
fn index() -> Template {
    Template::render("admin_home", context! {
        message: "Book Reviews - Assignment 3"
    })
}

#[get("/admin")]
fn admin_home() -> Template {
    Template::render("admin_home", context! {
        message: "Panel de Administración - Book Reviews"
    })
}

#[get("/admin/authors")]
fn admin_authors_list() -> Template {
    Template::render("authors_list", context! {})
}

#[get("/admin/books")]
fn admin_books_list() -> Template {
    Template::render("books_list", context! {})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            health,
            admin_home,
            admin_authors_list,
            admin_books_list
        ])
}
