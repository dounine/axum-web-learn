use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    port: Option<u16>,
    body_limit: Option<u64>,
    timeout: Option<u64>,
}

impl ServerConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3333)
    }
    pub fn body_limit(&self) -> u64 {
        self.body_limit.unwrap_or(10)
    }
    pub fn timeout(&self) -> u64 {
        self.timeout.unwrap_or(10)
    }
}
