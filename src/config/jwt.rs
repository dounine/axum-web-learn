use serde::Deserialize;

#[derive(Deserialize)]
pub struct JwtConfig {
    pub secret: Option<String>,
    pub expiration: Option<u64>,
    pub issuer: Option<String>,
    pub audience: Option<String>,
}

impl JwtConfig {
    pub fn secret(&self) -> String {
        self.secret.clone().unwrap_or("123456".to_string())
    }
    pub fn expiration(&self) -> u64 {
        self.expiration.unwrap_or(3600)
    }
    pub fn issuer(&self) -> String {
        self.issuer.clone().unwrap_or("localhost".to_string())
    }
    pub fn audience(&self) -> String {
        self.audience.clone().unwrap_or("localhost".to_string())
    }
}
