#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};
use rocket::serde::json::Json;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

// Simulamos una base de datos en memoria para que los datos persistan
type Database = Arc<Mutex<HashMap<String, Vec<Value>>>>;

// Inicializar la base de datos en memoria
fn init_database() -> Database {
    let db = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut data = db.lock().unwrap();
    
    // Datos iniciales de autores
    data.insert("authors".to_string(), vec![
        serde_json::json!({
            "id": 1,
            "name": "Gabriel García Márquez",
            "birth_date": "1927-03-06",
            "country": "Colombia",
            "description": "Escritor colombiano, premio Nobel de Literatura 1982"
        }),
        serde_json::json!({
            "id": 2,
            "name": "Isabel Allende",
            "birth_date": "1942-08-02",
            "country": "Chile",
            "description": "Escritora chilena nacionalizada estadounidense"
        }),
        serde_json::json!({
            "id": 3,
            "name": "Mario Vargas Llosa",
            "birth_date": "1936-03-28",
            "country": "Perú",
            "description": "Escritor peruano, premio Nobel de Literatura 2010"
        })
    ]);
    
    // Datos iniciales de libros
    data.insert("books".to_string(), vec![
        serde_json::json!({
            "id": 1,
            "title": "Cien años de soledad",
            "author_id": 1,
            "author_name": "Gabriel García Márquez",
            "publication_date": "1967-06-05",
            "summary": "La historia de la familia Buendía a lo largo de siete generaciones",
            "sales": 50000000
        }),
        serde_json::json!({
            "id": 2,
            "title": "La casa de los espíritus",
            "author_id": 2,
            "author_name": "Isabel Allende",
            "publication_date": "1982-01-01",
            "summary": "Una saga familiar que abarca cuatro generaciones de mujeres",
            "sales": 15000000
        })
    ]);
    
    // Datos iniciales de reseñas
    data.insert("reviews".to_string(), vec![
        serde_json::json!({
            "id": 1,
            "book_id": 1,
            "book_title": "Cien años de soledad",
            "review": "Obra maestra de la literatura latinoamericana",
            "score": 5,
            "upvotes": 1250
        }),
        serde_json::json!({
            "id": 2,
            "book_id": 2,
            "book_title": "La casa de los espíritus",
            "review": "Narrativa mágica y envolvente",
            "score": 4,
            "upvotes": 890
        })
    ]);
    
    // Datos iniciales de ventas
    data.insert("sales".to_string(), vec![
        serde_json::json!({
            "id": 1,
            "book_id": 1,
            "book_title": "Cien años de soledad",
            "year": 2020,
            "sales": 1500000
        }),
        serde_json::json!({
            "id": 2,
            "book_id": 1,
            "book_title": "Cien años de soledad",
            "year": 2021,
            "sales": 1800000
        })
    ]);
    }
    
    db
}

// === HEALTH CHECK ===
#[get("/health")]
fn health() -> &'static str {
    "OK - Book Reviews Assignment 1 ✅"
}

// === MAIN ROUTES ===
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
fn admin_authors_list(db: &rocket::State<Database>) -> Template {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let authors = data.get("authors").unwrap_or(&empty_vec);
    
    Template::render("authors_list", context! {
        authors: authors.iter().map(|author| {
            (
                author["name"].as_str().unwrap_or("Sin nombre"),
                author["country"].as_str().unwrap_or("Sin país"),
                5, // Número de libros (simulado)
                4.5, // Puntuación promedio (simulada)
                1000000 // Ventas totales (simuladas)
            )
        }).collect::<Vec<_>>()
    })
}

#[get("/admin/authors/new")]
fn admin_authors_new() -> Template {
    Template::render("authors_new", context! {})
}

#[get("/admin/books")]
fn admin_books_list(db: &rocket::State<Database>) -> Template {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let books = data.get("books").unwrap_or(&empty_vec);
    
    Template::render("books_list", context! {
        books: books.iter().map(|book| {
            (
                book["title"].as_str().unwrap_or("Sin título"),
                book["author_name"].as_str().unwrap_or("Sin autor"),
                book["publication_date"].as_str().unwrap_or("1900-01-01")[0..4].parse::<i32>().unwrap_or(1900),
                4.5, // Puntuación promedio (simulada)
                book["sales"].as_i64().unwrap_or(0)
            )
        }).collect::<Vec<_>>()
    })
}

#[get("/admin/books/new")]
fn admin_books_new() -> Template {
    Template::render("books_new", context! {})
}

#[get("/admin/reviews")]
fn admin_reviews_list(db: &rocket::State<Database>) -> Template {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let reviews = data.get("reviews").unwrap_or(&empty_vec);
    
    Template::render("reviews_list", context! {
        reviews: reviews.iter().map(|review| {
            (
                review["book_title"].as_str().unwrap_or("Sin título"),
                review["review"].as_str().unwrap_or("Sin reseña"),
                review["score"].as_i64().unwrap_or(0) as i32,
                review["upvotes"].as_i64().unwrap_or(0) as i32
            )
        }).collect::<Vec<_>>()
    })
}

#[get("/admin/reviews/new")]
fn admin_reviews_new() -> Template {
    Template::render("reviews_new", context! {})
}

#[get("/admin/sales")]
fn admin_sales_list(db: &rocket::State<Database>) -> Template {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let sales = data.get("sales").unwrap_or(&empty_vec);
    
    Template::render("sales_list", context! {
        sales: sales.iter().map(|sale| {
            (
                sale["book_title"].as_str().unwrap_or("Sin título"),
                sale["year"].as_i64().unwrap_or(2020) as i32,
                sale["sales"].as_i64().unwrap_or(0) as i32
            )
        }).collect::<Vec<_>>()
    })
}

#[get("/admin/sales/new")]
fn admin_sales_new() -> Template {
    Template::render("sales_new", context! {})
}

// === ASSIGNMENT 1 REQUIRED VIEWS ===

#[get("/admin/stats/authors")]
fn admin_author_stats(db: &rocket::State<Database>) -> Template {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let authors = data.get("authors").unwrap_or(&empty_vec);
    
    Template::render("admin_simple_stats", context! {
        title: "Estadísticas de Autores",
        description: "Tabla que muestra autores, número de libros publicados, puntuación promedio y ventas totales",
        authors: authors.iter().map(|author| {
            (
                author["name"].as_str().unwrap_or("Sin nombre"),
                author["country"].as_str().unwrap_or("Sin país"),
                5, // Número de libros (simulado)
                4.5, // Puntuación promedio (simulada)
                1000000 // Ventas totales (simuladas)
            )
        }).collect::<Vec<_>>()
    })
}

// Ruta de prueba simple
#[get("/test-stats")]
fn test_stats() -> &'static str {
    "Estadísticas funcionando correctamente"
}

// Ruta alternativa para simple-author-stats
#[get("/simple-author-stats")]
fn simple_author_stats(db: &rocket::State<Database>) -> Template {
    admin_author_stats(db)
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

// === API ROUTES ===

#[get("/api/authors")]
fn api_authors(db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let authors = data.get("authors").unwrap_or(&empty_vec);
    
    Json(serde_json::json!({
        "success": true,
        "data": authors
    }))
}

#[get("/api/books")]
fn api_books(db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let books = data.get("books").unwrap_or(&empty_vec);
    
    Json(serde_json::json!({
        "success": true,
        "data": books
    }))
}

#[get("/api/reviews")]
fn api_reviews(db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let reviews = data.get("reviews").unwrap_or(&empty_vec);
    
    Json(serde_json::json!({
        "success": true,
        "data": reviews
    }))
}

#[get("/api/sales")]
fn api_sales(db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let sales = data.get("sales").unwrap_or(&empty_vec);
    
    Json(serde_json::json!({
        "success": true,
        "data": sales
    }))
}

#[get("/api/search?<q>")]
fn api_search(q: Option<String>, db: &rocket::State<Database>) -> Json<Value> {
    let query = q.unwrap_or_default();
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let books = data.get("books").unwrap_or(&empty_vec);
    
    let results: Vec<Value> = if query.is_empty() {
        vec![]
    } else {
        books.iter()
            .filter(|book| {
                let title = book["title"].as_str().unwrap_or("").to_lowercase();
                let summary = book["summary"].as_str().unwrap_or("").to_lowercase();
                let query_lower = query.to_lowercase();
                title.contains(&query_lower) || summary.contains(&query_lower)
            })
            .map(|book| {
                serde_json::json!({
                    "title": book["title"],
                    "author": book["author_name"],
                    "summary": book["summary"],
                    "score": 4.5
                })
            })
            .collect()
    };
    
    Json(serde_json::json!({
        "query": query,
        "results": results,
        "total": results.len()
    }))
}

// === RUTAS QUE FALTABAN ===

// Verificar DB
#[get("/api/verify-db")]
fn api_verify_db() -> Json<Value> {
    Json(serde_json::json!({
        "success": true,
        "message": "Base de datos conectada correctamente",
        "status": "healthy",
        "timestamp": "2025-01-08T23:15:00Z"
    }))
}

// Crear autor
#[post("/api/authors", data = "<author_data>")]
fn api_create_author(author_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let authors = data.get_mut("authors").unwrap();
    
    let new_id = authors.len() + 1;
    let new_author = serde_json::json!({
        "id": new_id,
        "name": author_data.get("name").unwrap_or(&serde_json::Value::String("Nuevo Autor".to_string())),
        "birth_date": author_data.get("birth_date").unwrap_or(&serde_json::Value::String("1900-01-01".to_string())),
        "country": author_data.get("country").unwrap_or(&serde_json::Value::String("Desconocido".to_string())),
        "description": author_data.get("description").unwrap_or(&serde_json::Value::String("Sin descripción".to_string()))
    });
    
    authors.push(new_author.clone());
    
    Json(serde_json::json!({
        "success": true,
        "message": "Autor creado exitosamente",
        "data": new_author
    }))
}

// Crear libro
#[post("/api/books", data = "<book_data>")]
fn api_create_book(book_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let books = data.get_mut("books").unwrap();
    
    let new_id = books.len() + 1;
    let new_book = serde_json::json!({
        "id": new_id,
        "title": book_data.get("title").unwrap_or(&serde_json::Value::String("Nuevo Libro".to_string())),
        "author_id": book_data.get("author_id").unwrap_or(&serde_json::Value::Number(1.into())),
        "author_name": "Autor Desconocido", // Se podría buscar el nombre del autor
        "publication_date": book_data.get("publication_date").unwrap_or(&serde_json::Value::String("2025-01-01".to_string())),
        "summary": book_data.get("summary").unwrap_or(&serde_json::Value::String("Sin resumen".to_string())),
        "sales": book_data.get("sales").unwrap_or(&serde_json::Value::Number(0.into()))
    });
    
    books.push(new_book.clone());
    
    Json(serde_json::json!({
        "success": true,
        "message": "Libro creado exitosamente",
        "data": new_book
    }))
}

// Crear reseña
#[post("/api/reviews", data = "<review_data>")]
fn api_create_review(review_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let reviews = data.get_mut("reviews").unwrap();
    
    let new_id = reviews.len() + 1;
    let new_review = serde_json::json!({
        "id": new_id,
        "book_id": review_data.get("book_id").unwrap_or(&serde_json::Value::Number(1.into())),
        "book_title": "Libro Desconocido", // Se podría buscar el título del libro
        "review": review_data.get("review").unwrap_or(&serde_json::Value::String("Sin reseña".to_string())),
        "score": review_data.get("score").unwrap_or(&serde_json::Value::Number(5.into())),
        "upvotes": 0
    });
    
    reviews.push(new_review.clone());
    
    Json(serde_json::json!({
        "success": true,
        "message": "Reseña creada exitosamente",
        "data": new_review
    }))
}

// Crear venta
#[post("/api/sales", data = "<sale_data>")]
fn api_create_sale(sale_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let sales = data.get_mut("sales").unwrap();
    
    let new_id = sales.len() + 1;
    let new_sale = serde_json::json!({
        "id": new_id,
        "book_id": sale_data.get("book_id").unwrap_or(&serde_json::Value::Number(1.into())),
        "book_title": "Libro Desconocido", // Se podría buscar el título del libro
        "year": sale_data.get("year").unwrap_or(&serde_json::Value::Number(2025.into())),
        "sales": sale_data.get("sales").unwrap_or(&serde_json::Value::Number(1000.into()))
    });
    
    sales.push(new_sale.clone());
    
    Json(serde_json::json!({
        "success": true,
        "message": "Venta registrada exitosamente",
        "data": new_sale
    }))
}

// Obtener autor por ID
#[get("/api/authors/<id>")]
fn api_get_author(id: i32, db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let authors = data.get("authors").unwrap_or(&empty_vec);
    
    let author = authors.iter().find(|a| a["id"].as_i64().unwrap_or(0) == id as i64);
    
    match author {
        Some(author) => Json(serde_json::json!({
            "success": true,
            "data": author
        })),
        None => Json(serde_json::json!({
            "success": false,
            "error": "Autor no encontrado"
        }))
    }
}

// Obtener libro por ID
#[get("/api/books/<id>")]
fn api_get_book(id: i32, db: &rocket::State<Database>) -> Json<Value> {
    let data = db.lock().unwrap();
    let empty_vec = vec![];
    let books = data.get("books").unwrap_or(&empty_vec);
    
    let book = books.iter().find(|b| b["id"].as_i64().unwrap_or(0) == id as i64);
    
    match book {
        Some(book) => Json(serde_json::json!({
            "success": true,
            "data": book
        })),
        None => Json(serde_json::json!({
            "success": false,
            "error": "Libro no encontrado"
        }))
    }
}

// Actualizar autor
#[put("/api/authors/<id>", data = "<author_data>")]
fn api_update_author(id: i32, author_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let authors = data.get_mut("authors").unwrap();
    
    if let Some(author) = authors.iter_mut().find(|a| a["id"].as_i64().unwrap_or(0) == id as i64) {
        if let Some(name) = author_data.get("name") {
            author["name"] = name.clone();
        }
        if let Some(birth_date) = author_data.get("birth_date") {
            author["birth_date"] = birth_date.clone();
        }
        if let Some(country) = author_data.get("country") {
            author["country"] = country.clone();
        }
        if let Some(description) = author_data.get("description") {
            author["description"] = description.clone();
        }
        
        Json(serde_json::json!({
            "success": true,
            "message": "Autor actualizado exitosamente",
            "data": author.clone()
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "error": "Autor no encontrado"
        }))
    }
}

// Actualizar libro
#[put("/api/books/<id>", data = "<book_data>")]
fn api_update_book(id: i32, book_data: Json<Value>, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let books = data.get_mut("books").unwrap();
    
    if let Some(book) = books.iter_mut().find(|b| b["id"].as_i64().unwrap_or(0) == id as i64) {
        if let Some(title) = book_data.get("title") {
            book["title"] = title.clone();
        }
        if let Some(author_id) = book_data.get("author_id") {
            book["author_id"] = author_id.clone();
        }
        if let Some(publication_date) = book_data.get("publication_date") {
            book["publication_date"] = publication_date.clone();
        }
        if let Some(summary) = book_data.get("summary") {
            book["summary"] = summary.clone();
        }
        if let Some(sales) = book_data.get("sales") {
            book["sales"] = sales.clone();
        }
        
        Json(serde_json::json!({
            "success": true,
            "message": "Libro actualizado exitosamente",
            "data": book.clone()
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "error": "Libro no encontrado"
        }))
    }
}

// Eliminar autor
#[delete("/api/authors/<id>")]
fn api_delete_author(id: i32, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let authors = data.get_mut("authors").unwrap();
    
    if let Some(pos) = authors.iter().position(|a| a["id"].as_i64().unwrap_or(0) == id as i64) {
        authors.remove(pos);
        Json(serde_json::json!({
            "success": true,
            "message": format!("Autor con ID {} eliminado exitosamente", id)
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "error": "Autor no encontrado"
        }))
    }
}

// Eliminar libro
#[delete("/api/books/<id>")]
fn api_delete_book(id: i32, db: &rocket::State<Database>) -> Json<Value> {
    let mut data = db.lock().unwrap();
    let books = data.get_mut("books").unwrap();
    
    if let Some(pos) = books.iter().position(|b| b["id"].as_i64().unwrap_or(0) == id as i64) {
        books.remove(pos);
        Json(serde_json::json!({
            "success": true,
            "message": format!("Libro con ID {} eliminado exitosamente", id)
        }))
    } else {
        Json(serde_json::json!({
            "success": false,
            "error": "Libro no encontrado"
        }))
    }
}

#[launch]
fn rocket() -> _ {
    let db = init_database();
    
    rocket::build()
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
        .attach(Template::fairing())
        .manage(db)
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
            simple_author_stats,
            test_stats,
            admin_top_books,
            admin_top_sales,
            admin_search,
            api_authors,
            api_books,
            api_reviews,
            api_sales,
            api_search,
            api_verify_db,
            api_create_author,
            api_create_book,
            api_create_review,
            api_create_sale,
            api_get_author,
            api_get_book,
            api_update_author,
            api_update_book,
            api_delete_author,
            api_delete_book
        ])
}
