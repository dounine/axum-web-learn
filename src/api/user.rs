use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;
use tracing::debug;
use validator::Validate;

use crate::{
    api::response::ApiResponse,
    app::AppState,
    auth::{Auth, Jwt},
    common::PaginationParams,
    error::ApiError,
    handler::{valid_json::ValidJson, valid_query::ValidQuery},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/info", get(user_info).post(create_user))
        .route("/list", get(users))
        .route_layer(crate::middleware::auth::auth_layer())
        .route("/login", post(login))
}
#[derive(Debug, Validate, Deserialize)]
pub struct LoginParams {
    #[validate(length(min = 1))]
    username: String,
    #[validate(length(min = 1))]
    password: String,
}

pub async fn login(
    Extension(jwt): Extension<Arc<Jwt>>,
    ValidJson(params): ValidJson<LoginParams>,
) -> Result<ApiResponse<'static, String>, ApiError> {
    let token = jwt.encode(Auth {
        user: params.username,
    })?;
    Ok(ApiResponse {
        ok: true,
        msg: None,
        data: Some(token),
    })
}
pub async fn users(
    ValidQuery(params): ValidQuery<PaginationParams>,
    State(state): State<AppState>,
) -> ApiResponse<'static, ()> {
    debug!("params: {:?}", params);
    ApiResponse {
        ok: true,
        msg: None,
        data: None,
    }
}

pub async fn user_info(State(state): State<AppState>) -> ApiResponse<'static, ()> {
    ApiResponse {
        ok: true,
        msg: None,
        data: None,
    }
}

pub async fn create_user(
    State(state): State<AppState>,
) -> ApiResponse<'static, ()> {
    ApiResponse {
        ok: true,
        msg: None,
        data: None,
    }
}
