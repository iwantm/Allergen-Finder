use crate::models::auth;
use crate::models::error::{ApiError, ErrorStruct};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{catch, catchers, Catcher, Request};

#[catch(401)]
fn unauthorized(req: &Request) -> Json<ApiError<String>> {
    let auth_errors = req.local_cache::<ApiError<String>, _>(|| {
        ApiError::AuthenticationError(ErrorStruct::new("Unauthorized", None))
    });

    Json(auth_errors.clone())
}

#[catch(404)]
fn not_found() -> Json<ErrorStruct<String>> {
    Json(ErrorStruct::new("Not Found", None))
}

#[catch(500)]
fn internal_server_error(req: &Request) -> Json<ErrorStruct<String>> {
    Json(ErrorStruct::new(
        "Internal Server Error",
        Some(format!("{:?}", req)),
    ))
}

pub fn catchers() -> Vec<Catcher> {
    catchers![unauthorized, not_found, internal_server_error]
}
