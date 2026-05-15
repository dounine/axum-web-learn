use crate::error::ApiError;
use axum::extract::FromRequestParts;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidQuery<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let query = axum::extract::Query::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| ApiError::QueryParseError(e))?;
            query
                .0
                .validate()
                .map_err(|e| ApiError::ValidationError(e.to_string()))?;
            Ok(ValidQuery(query.0))
        }
    }
}
