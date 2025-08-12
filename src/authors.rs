use rocket::serde::json::Json;
use rocket::{get, post, put, delete, routes, Route};
use rocket::http::Status;
use rocket_db_pools::Connection;
use validator::Validate;
use chrono::Utc;
use sqlx;

use crate::models::{Author, CreateAuthor, UpdateAuthor, ApiResponse};
use crate::BookDb;

// ============================================================================
// CRUD OPERATIONS FOR AUTHORS
// ============================================================================

/// GET /authors - Get all authors
#[get("/authors")]
pub async fn get_all_authors(mut db: Connection<BookDb>) -> Result<Json<ApiResponse<Vec<Author>>>, Status> {
    match sqlx::query_as::<_, Author>(
        "SELECT id, name, date_of_birth, country_of_origin, description, created_at, updated_at 
         FROM authors 
         ORDER BY created_at DESC"
    )
    .fetch_all(&mut **db)
    .await
    {
        Ok(authors) => Ok(Json(ApiResponse::success("Authors retrieved successfully", authors))),
        Err(e) => {
            eprintln!("Database error getting authors: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// GET /authors/<id> - Get author by ID
#[get("/authors/<id>")]
pub async fn get_author_by_id(id: i32, mut db: Connection<BookDb>) -> Result<Json<ApiResponse<Author>>, Status> {
    match sqlx::query_as::<_, Author>(
        "SELECT id, name, date_of_birth, country_of_origin, description, created_at, updated_at 
         FROM authors 
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&mut **db)
    .await
    {
        Ok(Some(author)) => Ok(Json(ApiResponse::success("Author found", author))),
        Ok(None) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error getting author {}: {:?}", id, e);
            Err(Status::InternalServerError)
        }
    }
}

/// POST /authors - Create new author
#[post("/authors", data = "<new_author>")]
pub async fn create_author(
    new_author: Json<CreateAuthor>, 
    mut db: Connection<BookDb>
) -> Result<Json<ApiResponse<Author>>, Status> {
    // Validate input
    if let Err(validation_errors) = new_author.validate() {
        eprintln!("Validation errors: {:?}", validation_errors);
        return Err(Status::BadRequest);
    }

    let now = Utc::now();
    
    match sqlx::query_as::<_, Author>(
        "INSERT INTO authors (name, date_of_birth, country_of_origin, description, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, name, date_of_birth, country_of_origin, description, created_at, updated_at"
    )
    .bind(&new_author.name)
    .bind(&new_author.date_of_birth)
    .bind(&new_author.country_of_origin)
    .bind(&new_author.description)
    .bind(now)
    .bind(now)
    .fetch_one(&mut **db)
    .await
    {
        Ok(author) => Ok(Json(ApiResponse::success("Author created successfully", author))),
        Err(e) => {
            eprintln!("Database error creating author: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

/// PUT /authors/<id> - Update author
#[put("/authors/<id>", data = "<update_author>")]
pub async fn update_author(
    id: i32,
    update_author: Json<UpdateAuthor>, 
    mut db: Connection<BookDb>
) -> Result<Json<ApiResponse<Author>>, Status> {
    // Validate input
    if let Err(validation_errors) = update_author.validate() {
        eprintln!("Validation errors: {:?}", validation_errors);
        return Err(Status::BadRequest);
    }

    // First, check if the author exists
    let existing_author = match sqlx::query_as::<_, Author>(
        "SELECT id, name, date_of_birth, country_of_origin, description, created_at, updated_at 
         FROM authors WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&mut **db)
    .await
    {
        Ok(Some(author)) => author,
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error checking author existence: {:?}", e);
            return Err(Status::InternalServerError);
        }
    };

    // Build the update query dynamically based on provided fields
    let name = update_author.name.as_ref().unwrap_or(&existing_author.name);
    let date_of_birth = update_author.date_of_birth.or(existing_author.date_of_birth);
    let country_of_origin = update_author.country_of_origin.as_ref().or(existing_author.country_of_origin.as_ref());
    let description = update_author.description.as_ref().or(existing_author.description.as_ref());
    let updated_at = Utc::now();

    match sqlx::query_as::<_, Author>(
        "UPDATE authors 
         SET name = $1, date_of_birth = $2, country_of_origin = $3, description = $4, updated_at = $5
         WHERE id = $6
         RETURNING id, name, date_of_birth, country_of_origin, description, created_at, updated_at"
    )
    .bind(name)
    .bind(date_of_birth)
    .bind(country_of_origin)
    .bind(description)
    .bind(updated_at)
    .bind(id)
    .fetch_one(&mut **db)
    .await
    {
        Ok(author) => Ok(Json(ApiResponse::success("Author updated successfully", author))),
        Err(e) => {
            eprintln!("Database error updating author {}: {:?}", id, e);
            Err(Status::InternalServerError)
        }
    }
}

/// DELETE /authors/<id> - Delete author
#[delete("/authors/<id>")]
pub async fn delete_author(id: i32, mut db: Connection<BookDb>) -> Result<Json<ApiResponse<()>>, Status> {
    match sqlx::query("DELETE FROM authors WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(ApiResponse::success("Author deleted successfully", ())))
            } else {
                Err(Status::NotFound)
            }
        }
        Err(e) => {
            eprintln!("Error deleting author {}: {:?}", id, e);
            // Check if it's a foreign key constraint violation
            if e.to_string().contains("violates foreign key constraint") {
                Err(Status::Conflict) // Author has books, cannot delete
            } else {
                Err(Status::InternalServerError)
            }
        }
    }
}

// Export all routes
pub fn routes() -> Vec<Route> {
    routes![
        get_all_authors,
        get_author_by_id,
        create_author,
        update_author,
        delete_author
    ]
}
