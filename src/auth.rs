use std::{sync::LazyLock, time::Duration};

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{config::jwt::JwtConfig, error::ApiError};
const JWT: LazyLock<Jwt> = LazyLock::new(|| Jwt::new(&crate::config::get().jwt));
#[derive(Clone, Serialize, Deserialize)]
pub struct Auth {
    pub user: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    aut: Auth,
    iat: u64,
    exp: u64,
}

pub struct Jwt {
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl Jwt {
    pub fn new(config: &JwtConfig) -> Self {
        let mut validation = Validation::new(Algorithm::HS256); //创建验证器
        validation.set_audience(&[config.audience()]); //设置受众
        validation.set_issuer(&[config.issuer()]); //设置签发者
        validation.set_required_spec_claims(&["aut", "iat", "exp"]); //验证必须包含的字段

        Self {
            encode_secret: EncodingKey::from_secret(config.secret().as_bytes()),
            decode_secret: DecodingKey::from_secret(config.secret().as_bytes()),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: Duration::from_secs(config.expiration()),
            audience: config.audience().to_string(),
            issuer: config.issuer().to_string(),
        }
    }
    pub fn encode(&self, auth: Auth) -> Result<String, ApiError> {
        let current_timestamp = chrono::Utc::now().timestamp() as u64;
        let claims = Claims {
            aut: auth,
            iat: current_timestamp,
            exp: current_timestamp + self.expiration.as_secs() as u64,
        };
        Ok(encode(&self.header, &claims, &self.encode_secret)?)
    }
    pub fn decode(&self, token: &str) -> Result<Auth, ApiError> {
        let claims: Claims = decode(token, &self.decode_secret, &self.validation)?.claims;
        Ok(claims.aut)
    }
}
// pub fn get() -> &'static Jwt {
//     &JWT
// }
