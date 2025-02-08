use std::fmt::Debug;

use rocket::{response::Responder, serde::json::Error as SerdeError, serde::json::Json};

use diesel::result::{DatabaseErrorKind, Error as DieselError};
use tonic::Code;
use tonic::Status;

use super::generic::{ErrorMessage, JsonString};

#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    #[response(status = 400, content_type = "json")]
    BadRequest(JsonString),

    #[response(status = 401, content_type = "json")]
    Unauthorized(JsonString),

    #[response(status = 404, content_type = "json")]
    NotFound(JsonString),

    /// 408 Request Timeout
    #[response(status = 408, content_type = "json")]
    RequestTimeout(JsonString),

    /// 409 Conflict
    #[response(status = 409, content_type = "json")]
    Conflict(JsonString),

    /// 412 Precondition Failed
    #[response(status = 412, content_type = "json")]
    PreconditionFailed(JsonString),

    /// 422 Unprocessable Entity
    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(JsonString),

    /// 444 No Response
    #[response(status = 444, content_type = "json")]
    NoResponse(JsonString),

    /// 499 Client Closed Request
    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(JsonString),

    /// 500 Internal Server Error
    #[response(status = 500, content_type = "json")]
    InternalServerError(JsonString),

    /// 501 Not Implemented
    #[response(status = 501, content_type = "json")]
    NotImplemented(JsonString),

    /// 503 Service Unavailable
    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(JsonString),

    /// 511 Network Authentication Required
    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(JsonString),
}

pub type RestDto<'a, T> = Result<Json<T>, SerdeError<'a>>;

pub type RestResult<T> = Result<Json<T>, ErrorResponse>;

impl ErrorResponse {
    fn convert(status: Status) -> Self {
        let message = ErrorMessage::from(status.message()).json();

        match status.code() {
            Code::Aborted => Self::NoResponse(message),
            Code::AlreadyExists => Self::Conflict(message),
            Code::Cancelled => Self::ClientClosedRequest(message),
            Code::DataLoss => Self::BadRequest(message),
            Code::DeadlineExceeded => Self::RequestTimeout(message),
            Code::FailedPrecondition => Self::PreconditionFailed(message),
            Code::Internal => Self::InternalServerError(message),
            Code::InvalidArgument => Self::UnprocessableEntity(message),
            Code::NotFound => Self::NotFound(message),
            Code::Ok => panic!("Returned an error with an 'OK' status. What???"),
            Code::OutOfRange => Self::UnprocessableEntity(message),
            Code::PermissionDenied => Self::Unauthorized(message),
            Code::Unauthenticated => Self::NetworkAuthenticationRequired(message),
            Code::Unavailable => Self::ServiceUnavailable(message),
            Code::Unimplemented => Self::NotImplemented(message),
            Code::Unknown => Self::InternalServerError(message),
            _ => panic!("Unhandled return status: {}", status),
        }
    }

    fn convert_diesel_error(err: DieselError) -> Self {
        let message = ErrorMessage::from(err.to_string()).json();

        match err {
            DieselError::NotFound => Self::NotFound(message),
            DieselError::DatabaseError(error_kind, _) => match error_kind {
                DatabaseErrorKind::NotNullViolation => Self::BadRequest(message),
                DatabaseErrorKind::UniqueViolation => Self::Conflict(message),
                _ => panic!("Unhandled return status: {}", message),
            },

            _ => panic!("Unhandled return status: {}", message),
        }
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
