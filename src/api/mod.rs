use axum::{Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};

use crate::{app::AppState, error::ApiError};
pub mod ipa;
pub mod response;
pub mod user;
pub fn router() -> Router<AppState> {
    Router::new()
        .nest_service(
            "/signed.ipa",
            ServeFile::new("/Users/lake/dounine/github/ipa/fast-sign/data/signed.ipa"),
        )
        .nest_service(
            "/install.plist",
            ServeFile::new("/Users/lake/dounine/github/ipa/fast-sign/data/install.plist"),
        )
        .nest(
            "/api",
            Router::new()
                .nest("/user", user::router())
                .nest("/ipa", ipa::router()),
        )
        .fallback(async || -> Result<(), ApiError> { Err(ApiError::NotFound) })
}
