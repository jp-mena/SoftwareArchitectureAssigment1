use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use validator::Validate;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub country_of_origin: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAuthor {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
    #[validate(length(max = 100, message = "Country must be at most 100 characters"))]
    pub country_of_origin: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAuthor {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    #[validate(length(max = 100, message = "Country must be at most 100 characters"))]
    pub country_of_origin: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

// ===============================
// BOOKS MODELS
// ===============================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub summary: Option<String>,
    pub date_of_publication: Option<chrono::NaiveDate>,
    pub number_of_sales: i64,
    pub author_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 1, max = 500, message = "Name must be between 1 and 500 characters"))]
    pub name: String,
    #[validate(length(max = 2000, message = "Summary must be less than 2000 characters"))]
    pub summary: Option<String>,
    pub date_of_publication: Option<chrono::NaiveDate>,
    #[validate(range(min = 0, message = "Number of sales must be positive"))]
    pub number_of_sales: Option<i64>,
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateBook {
    #[validate(length(min = 1, max = 500, message = "Name must be between 1 and 500 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 2000, message = "Summary must be less than 2000 characters"))]
    pub summary: Option<String>,
    pub date_of_publication: Option<chrono::NaiveDate>,
    #[validate(range(min = 0, message = "Number of sales must be positive"))]
    pub number_of_sales: Option<i64>,
    pub author_id: Option<i32>,
}

// ===============================
// REVIEWS MODELS
// ===============================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Review {
    pub id: i32,
    pub book_id: i32,
    pub review: String,
    pub score: i32,
    pub number_of_upvotes: i32,
    pub reviewer_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateReview {
    pub book_id: i32,
    #[validate(length(min = 1, max = 2000, message = "Review must be between 1 and 2000 characters"))]
    pub review: String,
    #[validate(range(min = 1, max = 5, message = "Score must be between 1 and 5"))]
    pub score: i32,
    #[validate(length(max = 255, message = "Reviewer name must be less than 255 characters"))]
    pub reviewer_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateReview {
    #[validate(length(min = 1, max = 2000, message = "Review must be between 1 and 2000 characters"))]
    pub review: Option<String>,
    #[validate(range(min = 1, max = 5, message = "Score must be between 1 and 5"))]
    pub score: Option<i32>,
    #[validate(length(max = 255, message = "Reviewer name must be less than 255 characters"))]
    pub reviewer_name: Option<String>,
    #[validate(range(min = 0, message = "Number of upvotes must be positive"))]
    pub number_of_upvotes: Option<i32>,
}

// ===============================
// SALES MODELS
// ===============================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Sale {
    pub id: i32,
    pub book_id: i32,
    pub year: i32,
    pub sales: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateSale {
    pub book_id: i32,
    #[validate(range(min = 1900, max = 2100, message = "Year must be between 1900 and 2100"))]
    pub year: i32,
    #[validate(range(min = 0, message = "Sales must be positive"))]
    pub sales: i64,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateSale {
    #[validate(range(min = 1900, max = 2100, message = "Year must be between 1900 and 2100"))]
    pub year: Option<i32>,
    #[validate(range(min = 0, message = "Sales must be positive"))]
    pub sales: Option<i64>,
}
