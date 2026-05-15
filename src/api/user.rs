use axum::{Router, extract::State, routing::get};
use tracing::debug;

use crate::{
    api::response::ApiResponse, app::AppState, common::PaginationParams,
    handler::valid_query::ValidQuery,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/info", get(user_info).post(create_user))
        .route("/list", get(users))
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

pub async fn create_user(State(state): State<AppState>) -> ApiResponse<'static, ()> {
    ApiResponse {
        ok: true,
        msg: None,
        data: None,
    }
}
