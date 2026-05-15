use axum::{Json, extract::rejection::QueryRejection, http::StatusCode, response::IntoResponse};

use crate::api::response::ApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("config load error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("not found")]
    NotFound,
    #[error("server error: {0}")]
    ServerError(String),
    #[error("query parse error: {0}")]
    QueryParseError(#[from] QueryRejection),
    #[error("validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::ConfigError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::NotFound => (
                StatusCode::OK,
                Json(ApiResponse::<()>::err("url not found")),
            )
                .into_response(),
            ApiError::ServerError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::QueryParseError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::ValidationError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
        }
    }
}
