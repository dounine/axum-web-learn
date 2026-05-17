use std::{sync::Arc, time::Duration};

use axum::{
    Extension, Router,
    http::{Method, StatusCode},
};
use bytesize::ByteSize;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
};
use tracing::info;

use crate::{api, app::AppState, auth::Jwt, config::server::ServerConfig, error::ApiError};

pub struct Server {
    config: &'static ServerConfig,
}
impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }
    pub async fn run(&self, state: AppState) -> Result<(), ApiError> {
        // 构建路由
        let app = self.build_router(state);

        // 获取端口配置
        let app_port = self.config.port();
        let bind_addr = format!("0.0.0.0:{}", app_port);

        // 绑定监听地址
        let listener = tokio::net::TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| {
                ApiError::ServerError(format!("Failed to bind to {}: {}", bind_addr, e))
            })?;

        let addr = listener
            .local_addr()
            .map_err(|e| ApiError::ServerError(e.to_string()))?;

        info!("Server listening on http://{}", addr);

        // 启动服务
        axum::serve(listener, app)
            .await
            .map_err(|e| ApiError::ServerError(format!("Server error: {}", e)))?;

        Ok(())
    }

    pub fn build_router(&self, state: AppState) -> Router {
        let timeout = TimeoutLayer::with_status_code(
            StatusCode::OK,
            Duration::from_secs(self.config.timeout()),
        );

        let body_limit =
            RequestBodyLimitLayer::new(ByteSize::mb(self.config.body_limit()).as_u64() as usize);

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::DELETE])
            .allow_headers(Any)
            .max_age(Duration::from_hours(24));

        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        let jwt = Arc::new(Jwt::new(&crate::config::get().jwt));

        Router::new()
            .merge(api::router())
            .layer(Extension(jwt))
            .layer(timeout)
            .layer(body_limit)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}
