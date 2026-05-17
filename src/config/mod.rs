use std::sync::LazyLock;

use config::Config;
use serde::Deserialize;

use crate::error::ApiError;

pub mod jwt;
pub mod server;

static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::load().unwrap());
#[derive(Deserialize)]
pub struct AppConfig {
    pub server: server::ServerConfig,
    pub jwt: jwt::JwtConfig,
}
impl AppConfig {
    pub fn load() -> Result<Self, ApiError> {
        let app_config = Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()?
            .try_deserialize()?;
        Ok(app_config)
    }
}
pub fn get() -> &'static AppConfig {
    &APP_CONFIG
}
