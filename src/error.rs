use axum::{
    Json,
    body::Body,
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
    #[error("json parse error: {0}")]
    JsonParseError(#[from] JsonRejection),
    #[error("validation error: {0}")]
    ValidationError(String),
    #[error("auth error: {0}")]
    AuthError(#[from] jsonwebtoken::errors::Error),
    #[error("unauthorized")]
    Unauthorized(String),
    #[error("{0}")]
    Error(String),
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
            ApiError::JsonParseError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::AuthError(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::Unauthorized(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
            ApiError::Error(e) => {
                (StatusCode::OK, Json(ApiResponse::<()>::err(e.to_string()))).into_response()
            }
        }
    }
}

impl From<ApiError> for Response<Body> {
    fn from(value: ApiError) -> Self {
        value.into_response()
    }
}
