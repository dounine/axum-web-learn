use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<'a, T> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}
impl<'a, T> IntoResponse for ApiResponse<'a, T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
impl<'a, T> ApiResponse<'a, T> {
    pub fn ok(data: T) -> Self {
        Self {
            ok: true,
            msg: None,
            data: Some(data),
        }
    }
    pub fn err<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            ok: false,
            msg: Some(message.into()),
            data: None,
        }
    }
}
