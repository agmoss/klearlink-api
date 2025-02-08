use crate::generic::ErrorMessage;
use crate::response::ErrorResponse;
use rocket::catch;
use rocket::catchers;
use rocket::http::Status;
use rocket::Request;
pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![unauthorized, not_found,]
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
#[catch(404)]
fn not_found(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::NotFound(
        ErrorMessage::from(format!(
            "The resource you are looking for does not exist: {}",
            status.reason().unwrap(),
        ))
        .json(),
    )
}
