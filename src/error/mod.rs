use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    InternalServerError,

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::SqlxError(err) => {
                tracing::error!("SQLx error: {:?}", err);
                match &err {
                    sqlx::Error::Database(db_err) => {
                        match db_err.kind() {
                            sqlx::error::ErrorKind::UniqueViolation => (
                                StatusCode::CONFLICT,
                                "The data you entered already exists.".to_string(),
                            ),
                            sqlx::error::ErrorKind::ForeignKeyViolation => (
                                StatusCode::BAD_REQUEST,
                                "No related data found.".to_string(),
                            ),
                            sqlx::error::ErrorKind::NotNullViolation => (
                                StatusCode::BAD_REQUEST,
                                "There are required fields that are empty.".to_string(),
                            ),
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "An error occurred in the database.".to_string(),
                            ),
                        }
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "There is a problem connecting to the database.".to_string(),
                    ),
                }
            }
            AppError::JwtError(err) => {
                tracing::error!("JWT error: {:?}", err);
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid or expired token".to_string(),
                )
            }
            AppError::ValidationErrors(err) => {
                 let messages = err.field_errors().into_iter().map(|(_, errors)| {
                    errors.iter().map(|e| e.message.as_ref().unwrap().to_string()).collect::<Vec<_>>().join(", ")
                }).collect::<Vec<_>>().join("; ");
                (StatusCode::BAD_REQUEST, messages)
            }
            AppError::BcryptError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Password hashing error".to_string()),
        };

        let body = Json(ErrorResponse {
            code: status.as_u16(),
            message: error_message,
        });

        (status, body).into_response()
    }
}