use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
mod logger;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

async fn root() -> Json<serde_json::Value> {
    debug!("Hello, world!");
    Json(json!({
        "data":42
    }))
}
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::OK, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
#[tokio::main]
async fn main() {
    logger::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tracing::info!("Server listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
