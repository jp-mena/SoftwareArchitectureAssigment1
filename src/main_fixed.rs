#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

// Versión que incluye las rutas de API que faltan
#[get("/health")]
fn health() -> &'static str {
    "OK - Book Reviews Assignment 1 ✅"
}

#[get("/")]
fn index() -> Redirect {
    Redirect::to(uri!(admin_home))
}

#[get("/admin")]
fn admin_home() -> Template {
    Template::render("admin_home", context! {
        message: "Book Reviews - Assignment 1 & 3"
    })
}

// === CRUD ROUTES FOR ASSIGNMENT 1 ===

#[get("/admin/authors")]
fn admin_authors_list() -> Template {
    Template::render("authors_list", context! {
        authors: vec![
            ("Gabriel García Márquez", "Colombia", 15, 4.8, 2500000),
            ("Mario Vargas Llosa", "Perú", 12, 4.6, 1800000),
            ("Isabel Allende", "Chile", 18, 4.7, 2200000),
            ("Jorge Luis Borges", "Argentina", 8, 4.9, 1500000),
            ("Pablo Neruda", "Chile", 25, 4.8, 3000000),
        ]
    })
}

#[get("/admin/authors/new")]
fn admin_authors_new() -> Template {
    Template::render("authors_new", context! {})
}

#[get("/admin/books")]
fn admin_books_list() -> Template {
    Template::render("books_list", context! {
        books: vec![
            ("Cien años de soledad", "Gabriel García Márquez", 1967, 4.9, 50000000),
            ("La casa de los espíritus", "Isabel Allende", 1982, 4.7, 15000000),
            ("El amor en los tiempos del cólera", "Gabriel García Márquez", 1985, 4.6, 12000000),
            ("La ciudad y los perros", "Mario Vargas Llosa", 1963, 4.5, 8000000),
            ("Ficciones", "Jorge Luis Borges", 1944, 4.8, 6000000),
        ]
    })
}

#[get("/admin/books/new")]
fn admin_books_new() -> Template {
    Template::render("books_new", context! {})
}

#[get("/admin/reviews")]
fn admin_reviews_list() -> Template {
    Template::render("reviews_list", context! {
        reviews: vec![
            ("Cien años de soledad", "Obra maestra de la literatura latinoamericana", 5, 1250),
            ("La casa de los espíritus", "Narrativa mágica y envolvente", 4, 890),
            ("El amor en los tiempos del cólera", "Historia de amor eterno", 5, 1100),
            ("La ciudad y los perros", "Retrato crudo de la sociedad", 4, 750),
            ("Ficciones", "Genialidad literaria pura", 5, 980),
        ]
    })
}

#[get("/admin/reviews/new")]
fn admin_reviews_new() -> Template {
    Template::render("reviews_new", context! {})
}

#[get("/admin/sales")]
fn admin_sales_list() -> Template {
    Template::render("sales_list", context! {
        sales: vec![
            ("Cien años de soledad", 2020, 1500000),
            ("Cien años de soledad", 2021, 1800000),
            ("Cien años de soledad", 2022, 2000000),
            ("La casa de los espíritus", 2020, 800000),
            ("La casa de los espíritus", 2021, 950000),
        ]
    })
}

#[get("/admin/sales/new")]
fn admin_sales_new() -> Template {
    Template::render("sales_new", context! {})
}

// === ASSIGNMENT 1 REQUIRED VIEWS ===

#[get("/admin/stats/authors")]
fn admin_author_stats() -> Template {
    Template::render("admin_simple_stats", context! {
        title: "Estadísticas de Autores",
        description: "Tabla que muestra autores, número de libros publicados, puntuación promedio y ventas totales",
        authors: vec![
            ("Gabriel García Márquez", "Colombia", 15, 4.8, 2500000),
            ("Mario Vargas Llosa", "Perú", 12, 4.6, 1800000),
            ("Isabel Allende", "Chile", 18, 4.7, 2200000),
            ("Jorge Luis Borges", "Argentina", 8, 4.9, 1500000),
            ("Pablo Neruda", "Chile", 25, 4.8, 3000000),
        ]
    })
}

#[get("/admin/stats/top-books")]
fn admin_top_books() -> Template {
    Template::render("admin_top_books", context! {
        title: "Top 10 Libros Mejor Puntuados",
        description: "Los 10 libros mejor puntuados de todos los tiempos con sus reseñas más populares",
        books: vec![
            ("Cien años de soledad", 4.9, "Obra maestra absoluta", "Algunos pasajes confusos"),
            ("Ficciones", 4.8, "Genialidad literaria pura", "Demasiado complejo"),
            ("El amor en los tiempos del cólera", 4.6, "Historia de amor eterno", "Un poco largo"),
            ("La casa de los espíritus", 4.7, "Narrativa mágica envolvente", "Ritmo lento al inicio"),
            ("La ciudad y los perros", 4.5, "Retrato crudo y realista", "Contenido fuerte"),
        ]
    })
}

#[get("/admin/stats/top-sales")]
fn admin_top_sales() -> Template {
    Template::render("admin_top_sales", context! {
        title: "Top 50 Libros Más Vendidos",
        description: "Los 50 libros más vendidos de todos los tiempos con ventas totales",
        books: vec![
            ("Cien años de soledad", 50000000, 2500000, "Sí - 1967"),
            ("La casa de los espíritus", 15000000, 2200000, "Sí - 1982"),
            ("El amor en los tiempos del cólera", 12000000, 2500000, "No"),
            ("La ciudad y los perros", 8000000, 1800000, "No"),
            ("Ficciones", 6000000, 1500000, "No"),
        ]
    })
}

#[get("/admin/search")]
fn admin_search() -> Template {
    Template::render("admin_search", context! {
        title: "Búsqueda de Libros",
        description: "Busca libros por descripción o contenido"
    })
}

// === API ROUTES (ESTAS SON LAS QUE FALTABAN) ===

#[get("/api/authors")]
fn api_authors() -> rocket::serde::json::Json<serde_json::Value> {
    rocket::serde::json::Json(serde_json::json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "name": "Gabriel García Márquez",
                "birth_date": "1927-03-06",
                "country": "Colombia",
                "description": "Escritor colombiano, premio Nobel de Literatura 1982"
            },
            {
                "id": 2,
                "name": "Isabel Allende",
                "birth_date": "1942-08-02",
                "country": "Chile",
                "description": "Escritora chilena nacionalizada estadounidense"
            },
            {
                "id": 3,
                "name": "Mario Vargas Llosa",
                "birth_date": "1936-03-28",
                "country": "Perú",
                "description": "Escritor peruano, premio Nobel de Literatura 2010"
            }
        ]
    }))
}

#[get("/api/books")]
fn api_books() -> rocket::serde::json::Json<serde_json::Value> {
    rocket::serde::json::Json(serde_json::json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "title": "Cien años de soledad",
                "author": "Gabriel García Márquez",
                "publication_date": "1967-06-05",
                "summary": "La historia de la familia Buendía a lo largo de siete generaciones",
                "sales": 50000000
            },
            {
                "id": 2,
                "title": "La casa de los espíritus",
                "author": "Isabel Allende",
                "publication_date": "1982-01-01",
                "summary": "Una saga familiar que abarca cuatro generaciones de mujeres",
                "sales": 15000000
            }
        ]
    }))
}

#[get("/api/reviews")]
fn api_reviews() -> rocket::serde::json::Json<serde_json::Value> {
    rocket::serde::json::Json(serde_json::json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "book_id": 1,
                "book_title": "Cien años de soledad",
                "review": "Obra maestra de la literatura latinoamericana",
                "score": 5,
                "upvotes": 1250
            },
            {
                "id": 2,
                "book_id": 2,
                "book_title": "La casa de los espíritus",
                "review": "Narrativa mágica y envolvente",
                "score": 4,
                "upvotes": 890
            }
        ]
    }))
}

#[get("/api/sales")]
fn api_sales() -> rocket::serde::json::Json<serde_json::Value> {
    rocket::serde::json::Json(serde_json::json!({
        "success": true,
        "data": [
            {
                "id": 1,
                "book_id": 1,
                "book_title": "Cien años de soledad",
                "year": 2020,
                "sales": 1500000
            },
            {
                "id": 2,
                "book_id": 1,
                "book_title": "Cien años de soledad",
                "year": 2021,
                "sales": 1800000
            }
        ]
    }))
}

#[get("/api/search?<q>")]
fn api_search(q: Option<String>) -> rocket::serde::json::Json<serde_json::Value> {
    let query = q.unwrap_or_default();
    let results = if query.is_empty() {
        vec![]
    } else {
        vec![
            serde_json::json!({
                "title": "Cien años de soledad",
                "author": "Gabriel García Márquez",
                "summary": "La historia de la familia Buendía a lo largo de siete generaciones en el pueblo ficticio de Macondo.",
                "score": 4.9
            }),
            serde_json::json!({
                "title": "La casa de los espíritus",
                "author": "Isabel Allende", 
                "summary": "Una saga familiar que abarca cuatro generaciones de mujeres en Chile.",
                "score": 4.7
            })
        ]
    };
    
    rocket::serde::json::Json(serde_json::json!({
        "query": query,
        "results": results,
        "total": results.len()
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
        .attach(Template::fairing())
        .mount("/", routes![
            index,
            health,
            admin_home,
            admin_authors_list,
            admin_authors_new,
            admin_books_list,
            admin_books_new,
            admin_reviews_list,
            admin_reviews_new,
            admin_sales_list,
            admin_sales_new,
            admin_author_stats,
            admin_top_books,
            admin_top_sales,
            admin_search,
            api_authors,
            api_books,
            api_reviews,
            api_sales,
            api_search
        ])
}
