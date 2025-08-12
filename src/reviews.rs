use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{Connection, sqlx};
use validator::Validate;

use crate::{BookDb, models::{Review, CreateReview, UpdateReview, ApiResponse}};

// ===============================
// REVIEWS CRUD OPERATIONS
// ===============================

#[rocket::get("/reviews")]
pub async fn get_all_reviews(mut db: Connection<BookDb>) -> Result<Json<ApiResponse<Vec<Review>>>, Status> {
    let query = "
        SELECT id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at 
        FROM reviews 
        ORDER BY created_at DESC
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .fetch_all(&mut **db)
        .await
    {
        Ok(reviews) => Ok(Json(ApiResponse::success("Reviews retrieved successfully", reviews))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/reviews/<id>")]
pub async fn get_review_by_id(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<Review>>, Status> {
    let query = "
        SELECT id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at 
        FROM reviews 
        WHERE id = $1
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(review) => Ok(Json(ApiResponse::success("Review retrieved successfully", review))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/reviews/book/<book_id>")]
pub async fn get_reviews_by_book(mut db: Connection<BookDb>, book_id: i32) -> Result<Json<ApiResponse<Vec<Review>>>, Status> {
    let query = "
        SELECT id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at 
        FROM reviews 
        WHERE book_id = $1
        ORDER BY created_at DESC
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .bind(book_id)
        .fetch_all(&mut **db)
        .await
    {
        Ok(reviews) => Ok(Json(ApiResponse::success("Book reviews retrieved successfully", reviews))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::post("/reviews", data = "<review_data>")]
pub async fn create_review(
    mut db: Connection<BookDb>,
    review_data: Json<CreateReview>,
) -> Result<Json<ApiResponse<Review>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = review_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que el libro existe
    let book_check = sqlx::query_scalar::<_, i32>("SELECT id FROM books WHERE id = $1")
        .bind(review_data.book_id)
        .fetch_optional(&mut **db)
        .await;

    match book_check {
        Ok(Some(_)) => {}, // El libro existe
        Ok(None) => return Err(Status::BadRequest), // Libro no existe
        Err(e) => {
            eprintln!("Book check error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    let query = "
        INSERT INTO reviews (book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .bind(review_data.book_id)
        .bind(&review_data.review)
        .bind(review_data.score)
        .bind(0) // number_of_upvotes starts at 0
        .bind(&review_data.reviewer_name)
        .fetch_one(&mut **db)
        .await
    {
        Ok(review) => Ok(Json(ApiResponse::success("Review created successfully", review))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::put("/reviews/<id>", data = "<review_data>")]
pub async fn update_review(
    mut db: Connection<BookDb>,
    id: i32,
    review_data: Json<UpdateReview>,
) -> Result<Json<ApiResponse<Review>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = review_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que la review existe
    let existing_review = sqlx::query_as::<_, Review>("SELECT * FROM reviews WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    let current_review = match existing_review {
        Ok(Some(review)) => review,
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    };

    // Preparar valores para actualización
    let review_text = review_data.review.as_ref().unwrap_or(&current_review.review);
    let score = review_data.score.unwrap_or(current_review.score);
    let number_of_upvotes = review_data.number_of_upvotes.unwrap_or(current_review.number_of_upvotes);
    let reviewer_name = review_data.reviewer_name.as_ref().or(current_review.reviewer_name.as_ref());

    let query = "
        UPDATE reviews 
        SET review = $1, score = $2, number_of_upvotes = $3, reviewer_name = $4, updated_at = CURRENT_TIMESTAMP
        WHERE id = $5
        RETURNING id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .bind(review_text)
        .bind(score)
        .bind(number_of_upvotes)
        .bind(reviewer_name)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(review) => Ok(Json(ApiResponse::success("Review updated successfully", review))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::delete("/reviews/<id>")]
pub async fn delete_review(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<()>>, Status> {
    // Verificar que la review existe
    let exists = sqlx::query_scalar::<_, i32>("SELECT id FROM reviews WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    match exists {
        Ok(Some(_)) => {}, // La review existe
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    match sqlx::query("DELETE FROM reviews WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success("Review deleted successfully", ()))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::post("/reviews/<id>/upvote")]
pub async fn upvote_review(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<Review>>, Status> {
    let query = "
        UPDATE reviews 
        SET number_of_upvotes = number_of_upvotes + 1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING id, book_id, review, score, number_of_upvotes, reviewer_name, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Review>(query)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(review) => Ok(Json(ApiResponse::success("Review updated successfully", review))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

// Función para registrar las rutas
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_all_reviews,
        get_review_by_id,
        get_reviews_by_book,
        create_review,
        update_review,
        delete_review,
        upvote_review
    ]
}
