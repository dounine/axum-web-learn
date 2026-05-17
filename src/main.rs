mod api;
mod entity;
mod app;
mod config;
mod error;
mod logger;
mod server;
mod common;
mod handler;
mod auth;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    app::run().await?;

    Ok(())
}
