use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{auth::Jwt, error::ApiError, logger, server};

// 应用状态（线程安全）
#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Vec<String>>>,
    pub jwt: Arc<Jwt>,
}

pub async fn run() -> Result<(), ApiError> {
    // 初始化日志
    logger::init();

    // 创建应用状态
    let state = AppState {
        users: Arc::new(RwLock::new(vec![])),
        jwt: Arc::new(Jwt::new(&crate::config::get().jwt)),
    };

    server::Server::new(&crate::config::get().server)
        .run(state)
        .await?;

    Ok(())
}
