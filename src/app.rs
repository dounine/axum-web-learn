use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{error::ApiError, logger, server};

// 应用状态（线程安全）
#[derive(Clone)]
pub struct AppState {
    users: Arc<RwLock<Vec<String>>>,
    next_user_id: Arc<std::sync::atomic::AtomicU64>,
}

pub async fn run() -> Result<(), ApiError> {
    // 初始化日志
    logger::init();

    // 创建应用状态
    let state = AppState {
        users: Arc::new(RwLock::new(vec![])),
        next_user_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
    };

    server::Server::new(&crate::config::get().server)
        .run(state)
        .await?;

    Ok(())
}
