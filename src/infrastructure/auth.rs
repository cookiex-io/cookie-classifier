use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLASSIFIER_SECRET: String = std::env::var("CLASSIFIER_SECRET").expect("Must set CLASSIFIER_SECRET");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AuthError {
    ServiceTemporarilyUnavailable,
    InvalidAuthKey,
    MissingAuthKey,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::ServiceTemporarilyUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Oops! We're experiencing some technical issues. Please try again later."
            ),
            AuthError::InvalidAuthKey => (
                StatusCode::UNAUTHORIZED,
                "Invalid Auth Key in Header"
            ),
            AuthError::MissingAuthKey => (
                StatusCode::UNAUTHORIZED,
                "Missing Auth key in Header."
            ),
        };
        return (status,error_message).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthKey
    where
        S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
       let auth_key = parts.headers.get(AUTHORIZATION)
        .ok_or(AuthError::MissingAuthKey)?
        .to_str()
        .map_err(|_|AuthError::InvalidAuthKey)?;
       if auth_key == CLASSIFIER_SECRET.as_str() {
         return Ok(AuthKey)
       }
       Err(AuthError::InvalidAuthKey)
    }
}