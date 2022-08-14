use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use pj_core::error::DomainError;




pub enum AppError {
    DomainError(DomainError),
    SqlxError(sqlx::Error),
    ParseError()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::SqlxError(error)
    }
}

impl From<sqlx::types::uuid::Error> for AppError {
    fn from(_error: sqlx::types::uuid::Error) -> Self {
        Self::ParseError()
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        Self::DomainError(error)
    }
}
