use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    DatabaseError(String),
    ConnectionError,
    UniqueViolation,
    InternalServerError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: String,
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::NotFound => ApiError::NotFound,
            DieselError::DatabaseError(_, _) => {
                ApiError::DatabaseError("Database query failed".to_string())
            }
            DieselError::InvalidCString(_) => {
                ApiError::DatabaseError("Invalid database string".to_string())
            }
            DieselError::QueryBuilderError(_) => {
                ApiError::DatabaseError("Query building failed".to_string())
            }
            DieselError::DeserializationError(_) => {
                ApiError::DatabaseError("Failed to deserialize result".to_string())
            }
            DieselError::RollbackTransaction => {
                ApiError::InternalServerError("Transaction was rolled back".to_string())
            }
            DieselError::AlreadyInTransaction => {
                ApiError::InternalServerError("Already in a transaction".to_string())
            }
            _ => ApiError::InternalServerError("Unknown error".to_owned()),
        }
    }
}

impl Responder<'_, 'static> for ApiError {
    fn respond_to(self, _request: &Request) -> rocket::response::Result<'static> {
        let (status, message) = match self {
            ApiError::NotFound => (Status::NotFound, "Resource not found".to_string()),
            ApiError::DatabaseError(msg) => (Status::BadRequest, msg),
            ApiError::ConnectionError => (
                Status::ServiceUnavailable,
                "Database connection error".to_string(),
            ),
            ApiError::UniqueViolation => {
                (Status::Conflict, "Unique constraint violated".to_string())
            }
            ApiError::InternalServerError(msg) => (Status::InternalServerError, msg),
        };

        let error_response = ErrorResponse {
            status: "error",
            message,
        };

        let json_response = serde_json::to_string(&error_response).unwrap();
        Response::build()
            .header(rocket::http::ContentType::JSON)
            .status(status)
            .sized_body(json_response.len(), Cursor::new(json_response))
            .ok()
    }
}
