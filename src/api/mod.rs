use axum::Router;

use crate::{app::AppState, error::ApiError};

pub mod response;
pub mod user;
pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/api", Router::new().nest("/user", user::router()))
        .fallback(async || -> Result<(), ApiError> { Err(ApiError::NotFound) })
}
