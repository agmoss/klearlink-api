use crate::generic::{ErrorMessage, JsonString};
use rocket::serde::json::Error as SerdeError;
use rocket::{http::Status, response::Responder, serde::json::Json};

use diesel::result::Error as DieselError;
#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    #[response(status = 400, content_type = "json")]
    BadRequest(JsonString),

    #[response(status = 401, content_type = "json")]
    Unauthorized(JsonString),

    #[response(status = 404, content_type = "json")]
    NotFound(JsonString),

    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(JsonString),

    #[response(status = 444, content_type = "json")]
    NoResponse(JsonString),

    #[response(status = 500, content_type = "json")]
    InternalServerError(JsonString),

    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(JsonString),
}

pub type RestDto<'a, T> = Result<Json<T>, SerdeError<'a>>;

pub type RestResult<T> = Result<Json<T>, ErrorResponse>;

impl ErrorResponse {
    fn convert(status: Status) -> Self {
        let message = ErrorMessage::from(status.reason_lossy()).json();

        match status.code {
            401 => Self::Unauthorized(message),
            404 => Self::NotFound(message),
            422 => Self::UnprocessableEntity(message),
            _ => panic!("Unhandled return status: {}", status),
        }
    }

    fn convert_diesel_error(err: DieselError) -> Self {
        let message = ErrorMessage::from(err.to_string()).json();
        Self::NoResponse(message)
    }

    fn convert_serde_error(err: SerdeError) -> Self {
        let message = ErrorMessage::from(err.to_string()).json();
        Self::UnprocessableEntity(message)
    }
}

impl From<Status> for ErrorResponse {
    fn from(status: Status) -> Self {
        Self::convert(status)
    }
}

impl From<DieselError> for ErrorResponse {
    fn from(error: DieselError) -> ErrorResponse {
        Self::convert_diesel_error(error)
    }
}

impl From<SerdeError<'_>> for ErrorResponse {
    fn from(error: SerdeError<'_>) -> ErrorResponse {
        Self::convert_serde_error(error)
    }
}
