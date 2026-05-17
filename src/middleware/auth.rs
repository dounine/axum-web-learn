use std::{
    pin::Pin,
    sync::{Arc, LazyLock},
};

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderValue, header},
    response::Response,
};
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

use crate::{app::AppState, auth::Jwt, error::ApiError};

#[derive(Clone)]
pub struct AuthLayer {}

impl AsyncAuthorizeRequest<Body> for AuthLayer {
    type RequestBody = Body;

    type ResponseBody = Body;

    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send,
        >,
    >;

    fn authorize(&mut self, mut request: axum::http::Request<Body>) -> Self::Future {
        Box::pin(async move {
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value: &HeaderValue| {
                    value
                        .to_str()
                        .map_err(|e| {
                            ApiError::Unauthorized("authorization value is invalid".to_string())
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or(ApiError::Unauthorized(
                            "token not start with Bearer".to_string(),
                        ))
                })
                .transpose()?
                .ok_or(ApiError::Unauthorized(
                    "authorization header is missing".to_string(),
                ))?;
            let jwt = request
                .extensions()
                .get::<Arc<Jwt>>()
                .ok_or(ApiError::Error("jwt not set".to_string()))?;
            let auth = jwt.decode(token)?;
            request.extensions_mut().insert(auth);
            Ok(request)
        })
    }
}

pub fn auth_layer() -> AsyncRequireAuthorizationLayer<AuthLayer> {
    AsyncRequireAuthorizationLayer::new(AuthLayer {})
}
