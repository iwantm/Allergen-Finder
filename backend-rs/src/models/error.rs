use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Clone)]
pub enum ApiError<T> {
    #[error("Internal Server Error: {0}")]
    InternalServerError(ErrorStruct<T>),
    #[error("Not Found: {0}")]
    NotFound(ErrorStruct<T>),
    #[error("Authentication error: {0}")]
    AuthenticationError(ErrorStruct<T>),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ErrorStruct<T> {
    pub message: String,
    pub context: Option<T>,
}

impl<T: std::fmt::Debug> ErrorStruct<T> {
    pub fn new(message: &str, context: Option<T>) -> Self {
        Self {
            message: message.to_string(),
            context,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiError<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match &self {
            ApiError::InternalServerError(_) => Status::InternalServerError,
            ApiError::NotFound(_) => Status::NotFound,
            ApiError::AuthenticationError(_) => Status::Unauthorized,
        };

        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(body.len(), std::io::Cursor::new(body))
            .ok()
    }
}
