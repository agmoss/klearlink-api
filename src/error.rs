use crate::generic::ErrorMessage;
use crate::response::ErrorResponse;
use rocket::catch;
use rocket::http::Status;
use rocket::Request;

use rocket::catchers;

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![
        default_catcher,
        bad_request,
        unauthorized,
        not_found,
        unprocessable_entity,
        internal,
        service_unavailable
    ]
}

#[catch(default)]
fn default_catcher(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::InternalServerError(
        ErrorMessage::from(format!("Error while performing request: {}", status)).json(),
    )
}

#[catch(400)]
fn bad_request(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::BadRequest(
        ErrorMessage::from(format!(
            "The request is not valid: {}",
            status.reason().unwrap()
        ))
        .json(),
    )
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

#[catch(422)]
fn unprocessable_entity(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::UnprocessableEntity(
        ErrorMessage::from(format!(
            "The provided data could not be processed: {}",
            status.reason().unwrap(),
        ))
        .json(),
    )
}

#[catch(500)]
fn internal(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::InternalServerError(
        ErrorMessage::from(format!(
            "There was a problem in the service while processing your request: {}",
            status.reason().unwrap(),
        ))
        .json(),
    )
}

#[catch(503)]
fn service_unavailable(status: Status, _req: &Request) -> ErrorResponse {
    ErrorResponse::ServiceUnavailable(
        ErrorMessage::from(format!(
            "The service is currently unavailable: {}",
            status.reason().unwrap(),
        ))
        .json(),
    )
}
