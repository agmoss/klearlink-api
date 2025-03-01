use crate::core::{reqres::ErrorMessage, reqres::ErrorResponse};
use rocket::{catch, catchers, http::Status, serde::json::Json, Request};

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![unauthorized]
}

#[catch(401)]
fn unauthorized(status: Status, _req: &Request) -> ErrorResponse {
    let message = ErrorMessage::from_str(&format!(
        "This request is unauthorized: {}",
        status.reason().unwrap_or("Unknown reason"),
    ));
    ErrorResponse::Unauthorized(Json(message))
}
