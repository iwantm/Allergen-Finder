use rocket::{serde::json::Json, Responder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Responder, Error, Debug)]
pub enum ApiError<T> {
    #[error("Internal Server Error: {0}")]
    #[response(status = 500, content_type = "json")]
    InternalServer(Json<T>),
    #[error("Not Found: {0}")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<T>),
}

#[derive(Responder, Error, Debug)]
pub enum AuthError<T> {
    #[error("Failed to retrieve JWKs: {0}")]
    #[response(status = 500, content_type = "json")]
    JwkError(Json<T>),
    #[error("Unauthorised: {0}")]
    #[response(status = 401, content_type = "json")]
    TokenError(Json<T>),
    #[error("Auth0 error: {0}")]
    #[response(status = 401, content_type = "json")]
    Auth0Error(Json<T>),
    #[error("Auth0 error: {0}")]
    #[response(status = 401, content_type = "json")]
    JwtError(Json<T>),
}

#[derive(Deserialize, Serialize)]
pub struct ErrorStruct {
    message: String,
}
