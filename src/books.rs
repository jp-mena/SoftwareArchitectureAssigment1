use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{Connection, sqlx};
use validator::Validate;

use crate::{BookDb, models::{Book, CreateBook, UpdateBook, ApiResponse}};

// ===============================
// BOOKS CRUD OPERATIONS
// ===============================

#[rocket::get("/books")]
pub async fn get_all_books(mut db: Connection<BookDb>) -> Result<Json<ApiResponse<Vec<Book>>>, Status> {
    let query = "
        SELECT id, name, summary, date_of_publication, number_of_sales, author_id, created_at, updated_at 
        FROM books 
        ORDER BY created_at DESC
    ";
    
    match sqlx::query_as::<_, Book>(query)
        .fetch_all(&mut **db)
        .await
    {
        Ok(books) => Ok(Json(ApiResponse::success("Books retrieved successfully", books))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/books/<id>")]
pub async fn get_book_by_id(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<Book>>, Status> {
    let query = "
        SELECT id, name, summary, date_of_publication, number_of_sales, author_id, created_at, updated_at 
        FROM books 
        WHERE id = $1
    ";
    
    match sqlx::query_as::<_, Book>(query)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(book) => Ok(Json(ApiResponse::success("Book retrieved successfully", book))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::post("/books", data = "<book_data>")]
pub async fn create_book(
    mut db: Connection<BookDb>,
    book_data: Json<CreateBook>,
) -> Result<Json<ApiResponse<Book>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = book_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que el autor existe
    let author_check = sqlx::query_scalar::<_, i32>("SELECT id FROM authors WHERE id = $1")
        .bind(book_data.author_id)
        .fetch_optional(&mut **db)
        .await;

    match author_check {
        Ok(Some(_)) => {}, // El autor existe, continuar
        Ok(None) => return Err(Status::BadRequest), // Autor no existe
        Err(e) => {
            eprintln!("Author check error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    let query = "
        INSERT INTO books (name, summary, date_of_publication, number_of_sales, author_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, name, summary, date_of_publication, number_of_sales, author_id, created_at, updated_at
    ";
    
    let number_of_sales = book_data.number_of_sales.unwrap_or(0);
    
    match sqlx::query_as::<_, Book>(query)
        .bind(&book_data.name)
        .bind(&book_data.summary)
        .bind(&book_data.date_of_publication)
        .bind(number_of_sales)
        .bind(book_data.author_id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(book) => Ok(Json(ApiResponse::success("Book created successfully", book))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::put("/books/<id>", data = "<book_data>")]
pub async fn update_book(
    mut db: Connection<BookDb>,
    id: i32,
    book_data: Json<UpdateBook>,
) -> Result<Json<ApiResponse<Book>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = book_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que el libro existe
    let existing_book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    let current_book = match existing_book {
        Ok(Some(book)) => book,
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    };

    // Si se proporciona un nuevo author_id, verificar que existe
    if let Some(new_author_id) = book_data.author_id {
        let author_check = sqlx::query_scalar::<_, i32>("SELECT id FROM authors WHERE id = $1")
            .bind(new_author_id)
            .fetch_optional(&mut **db)
            .await;

        match author_check {
            Ok(Some(_)) => {}, // El autor existe
            Ok(None) => return Err(Status::BadRequest), // Autor no existe
            Err(e) => {
                eprintln!("Author check error: {e:?}");
                return Err(Status::InternalServerError);
            }
        }
    }

    // Preparar valores para actualización (usar valores actuales si no se proporcionan nuevos)
    let name = book_data.name.as_ref().unwrap_or(&current_book.name);
    let summary = book_data.summary.as_ref().or(current_book.summary.as_ref());
    let date_of_publication = book_data.date_of_publication.or(current_book.date_of_publication);
    let number_of_sales = book_data.number_of_sales.unwrap_or(current_book.number_of_sales);
    let author_id = book_data.author_id.unwrap_or(current_book.author_id);

    let query = "
        UPDATE books 
        SET name = $1, summary = $2, date_of_publication = $3, number_of_sales = $4, 
            author_id = $5, updated_at = CURRENT_TIMESTAMP
        WHERE id = $6
        RETURNING id, name, summary, date_of_publication, number_of_sales, author_id, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Book>(query)
        .bind(name)
        .bind(summary)
        .bind(date_of_publication)
        .bind(number_of_sales)
        .bind(author_id)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(book) => Ok(Json(ApiResponse::success("Book updated successfully", book))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::delete("/books/<id>")]
pub async fn delete_book(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<()>>, Status> {
    // Verificar que el libro existe
    let exists = sqlx::query_scalar::<_, i32>("SELECT id FROM books WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    match exists {
        Ok(Some(_)) => {}, // El libro existe
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    match sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success("Book deleted successfully", ()))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

// Función para registrar las rutas
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_all_books,
        get_book_by_id,
        create_book,
        update_book,
        delete_book
    ]
}
