use crate::generic::ErrorMessage;
use crate::response::ErrorResponse;
use rocket::{catch, catchers, http::Status, Request};

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![unauthorized]
}

#[catch(401)]
fn unauthorized(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::Unauthorized(
        ErrorMessage::from(format!(
            "This request is unauthorized: {}",
            status.reason().unwrap(),
        ))
        .json(),
    )
}
