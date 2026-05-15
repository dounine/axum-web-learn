use axum::extract::FromRequest;
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::ApiError;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidJson<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request(
        req: axum::extract::Request,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let json = axum::extract::Json::<T>::from_request(req, state).await?;
            json.0
                .validate()
                .map_err(|e| ApiError::ValidationError(e.to_string()))?;
            Ok(ValidJson(json.0))
        }
    }
}
