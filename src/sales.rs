use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::{Connection, sqlx};
use validator::Validate;

use crate::{BookDb, models::{Sale, CreateSale, UpdateSale, ApiResponse}};

// ===============================
// SALES CRUD OPERATIONS
// ===============================

#[rocket::get("/sales")]
pub async fn get_all_sales(mut db: Connection<BookDb>) -> Result<Json<ApiResponse<Vec<Sale>>>, Status> {
    let query = "
        SELECT id, book_id, year, sales, created_at, updated_at 
        FROM sales 
        ORDER BY year DESC, sales DESC
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .fetch_all(&mut **db)
        .await
    {
        Ok(sales) => Ok(Json(ApiResponse::success("Sales retrieved successfully", sales))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/sales/<id>")]
pub async fn get_sale_by_id(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<Sale>>, Status> {
    let query = "
        SELECT id, book_id, year, sales, created_at, updated_at 
        FROM sales 
        WHERE id = $1
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(sale) => Ok(Json(ApiResponse::success("Sale operation successful", sale))),
        Err(sqlx::Error::RowNotFound) => Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/sales/book/<book_id>")]
pub async fn get_sales_by_book(mut db: Connection<BookDb>, book_id: i32) -> Result<Json<ApiResponse<Vec<Sale>>>, Status> {
    let query = "
        SELECT id, book_id, year, sales, created_at, updated_at 
        FROM sales 
        WHERE book_id = $1
        ORDER BY year DESC
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .bind(book_id)
        .fetch_all(&mut **db)
        .await
    {
        Ok(sales) => Ok(Json(ApiResponse::success("Sales retrieved successfully", sales))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::get("/sales/year/<year>")]
pub async fn get_sales_by_year(mut db: Connection<BookDb>, year: i32) -> Result<Json<ApiResponse<Vec<Sale>>>, Status> {
    let query = "
        SELECT id, book_id, year, sales, created_at, updated_at 
        FROM sales 
        WHERE year = $1
        ORDER BY sales DESC
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .bind(year)
        .fetch_all(&mut **db)
        .await
    {
        Ok(sales) => Ok(Json(ApiResponse::success("Sales retrieved successfully", sales))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::post("/sales", data = "<sale_data>")]
pub async fn create_sale(
    mut db: Connection<BookDb>,
    sale_data: Json<CreateSale>,
) -> Result<Json<ApiResponse<Sale>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = sale_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que el libro existe
    let book_check = sqlx::query_scalar::<_, i32>("SELECT id FROM books WHERE id = $1")
        .bind(sale_data.book_id)
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

    // Verificar que no existe ya un registro para ese libro y año
    let existing_sale = sqlx::query_scalar::<_, i32>("SELECT id FROM sales WHERE book_id = $1 AND year = $2")
        .bind(sale_data.book_id)
        .bind(sale_data.year)
        .fetch_optional(&mut **db)
        .await;

    match existing_sale {
        Ok(Some(_)) => return Err(Status::Conflict), // Ya existe un registro para este libro y año
        Ok(None) => {}, // No existe, podemos proceder
        Err(e) => {
            eprintln!("Duplicate check error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    let query = "
        INSERT INTO sales (book_id, year, sales, created_at, updated_at)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id, book_id, year, sales, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .bind(sale_data.book_id)
        .bind(sale_data.year)
        .bind(sale_data.sales)
        .fetch_one(&mut **db)
        .await
    {
        Ok(sale) => Ok(Json(ApiResponse::success("Sale operation successful", sale))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::put("/sales/<id>", data = "<sale_data>")]
pub async fn update_sale(
    mut db: Connection<BookDb>,
    id: i32,
    sale_data: Json<UpdateSale>,
) -> Result<Json<ApiResponse<Sale>>, Status> {
    // Validar datos de entrada
    if let Err(validation_errors) = sale_data.validate() {
        eprintln!("Validation errors: {validation_errors:?}");
        return Err(Status::BadRequest);
    }

    // Verificar que la venta existe
    let existing_sale = sqlx::query_as::<_, Sale>("SELECT * FROM sales WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    let current_sale = match existing_sale {
        Ok(Some(sale)) => sale,
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    };

    // Si se actualiza el año, verificar que no entre en conflicto con otro registro
    if let Some(new_year) = sale_data.year {
        if new_year != current_sale.year {
            let conflict_check = sqlx::query_scalar::<_, i32>("SELECT id FROM sales WHERE book_id = $1 AND year = $2 AND id != $3")
                .bind(current_sale.book_id)
                .bind(new_year)
                .bind(id)
                .fetch_optional(&mut **db)
                .await;

            match conflict_check {
                Ok(Some(_)) => return Err(Status::Conflict), // Ya existe otro registro para este libro y año
                Ok(None) => {}, // No hay conflicto
                Err(e) => {
                    eprintln!("Conflict check error: {e:?}");
                    return Err(Status::InternalServerError);
                }
            }
        }
    }

    // Preparar valores para actualización
    let year = sale_data.year.unwrap_or(current_sale.year);
    let sales = sale_data.sales.unwrap_or(current_sale.sales);

    let query = "
        UPDATE sales 
        SET year = $1, sales = $2, updated_at = CURRENT_TIMESTAMP
        WHERE id = $3
        RETURNING id, book_id, year, sales, created_at, updated_at
    ";
    
    match sqlx::query_as::<_, Sale>(query)
        .bind(year)
        .bind(sales)
        .bind(id)
        .fetch_one(&mut **db)
        .await
    {
        Ok(sale) => Ok(Json(ApiResponse::success("Sale operation successful", sale))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

#[rocket::delete("/sales/<id>")]
pub async fn delete_sale(mut db: Connection<BookDb>, id: i32) -> Result<Json<ApiResponse<()>>, Status> {
    // Verificar que la venta existe
    let exists = sqlx::query_scalar::<_, i32>("SELECT id FROM sales WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut **db)
        .await;

    match exists {
        Ok(Some(_)) => {}, // La venta existe
        Ok(None) => return Err(Status::NotFound),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            return Err(Status::InternalServerError);
        }
    }

    match sqlx::query("DELETE FROM sales WHERE id = $1")
        .bind(id)
        .execute(&mut **db)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse::success("Sale deleted successfully", ()))),
        Err(e) => {
            eprintln!("Database error: {e:?}");
            Err(Status::InternalServerError)
        }
    }
}

// Función para registrar las rutas
pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_all_sales,
        get_sale_by_id,
        get_sales_by_book,
        get_sales_by_year,
        create_sale,
        update_sale,
        delete_sale
    ]
}
