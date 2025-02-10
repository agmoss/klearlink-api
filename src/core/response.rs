use diesel::result::{DatabaseErrorKind, Error as DieselError};
use rocket::{response::Responder, serde::json::Error as SerdeError, serde::json::Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_valid::validation::Errors as SerdeValidErrors;
use std::fmt::Debug;
use tonic::{Code, Status};

#[derive(Serialize, Deserialize, Clone, Debug, Responder)]
pub struct ErrorMessage {
    pub error: Value,
}

impl ErrorMessage {
    // Constructor for a string message
    pub fn from_str(msg: &str) -> Self {
        Self {
            error: Value::String(msg.to_owned()),
        }
    }

    // Constructor for a serde_json::Value
    pub fn from_value(value: Value) -> Self {
        Self { error: value }
    }
}

#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorMessage>),

    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<ErrorMessage>),

    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorMessage>),

    #[response(status = 408, content_type = "json")]
    RequestTimeout(Json<ErrorMessage>),

    #[response(status = 409, content_type = "json")]
    Conflict(Json<ErrorMessage>),

    #[response(status = 412, content_type = "json")]
    PreconditionFailed(Json<ErrorMessage>),

    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(Json<ErrorMessage>),

    #[response(status = 444, content_type = "json")]
    NoResponse(Json<ErrorMessage>),

    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(Json<ErrorMessage>),

    #[response(status = 500, content_type = "json")]
    InternalServerError(Json<ErrorMessage>),

    #[response(status = 501, content_type = "json")]
    NotImplemented(Json<ErrorMessage>),

    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(Json<ErrorMessage>),

    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(Json<ErrorMessage>),
}

pub type RestDto<'a, T> = Result<Json<T>, SerdeError<'a>>;

pub type RestResult<T> = Result<Json<T>, ErrorResponse>;

impl ErrorResponse {
    fn convert(status: Status) -> Self {
        let message = ErrorMessage::from_str(status.message());
        match status.code() {
            Code::Aborted => Self::NoResponse(Json(message)),
            Code::AlreadyExists => Self::Conflict(Json(message)),
            Code::Cancelled => Self::ClientClosedRequest(Json(message)),
            Code::DataLoss => Self::BadRequest(Json(message)),
            Code::DeadlineExceeded => Self::RequestTimeout(Json(message)),
            Code::FailedPrecondition => Self::PreconditionFailed(Json(message)),
            Code::Internal => Self::InternalServerError(Json(message)),
            Code::InvalidArgument => Self::UnprocessableEntity(Json(message)),
            Code::NotFound => Self::NotFound(Json(message)),
            Code::Ok => panic!("Returned an error with an 'OK' status. What???"),
            Code::OutOfRange => Self::UnprocessableEntity(Json(message)),
            Code::PermissionDenied => Self::Unauthorized(Json(message)),
            Code::Unauthenticated => Self::NetworkAuthenticationRequired(Json(message)),
            Code::Unavailable => Self::ServiceUnavailable(Json(message)),
            Code::Unimplemented => Self::NotImplemented(Json(message)),
            Code::Unknown => Self::InternalServerError(Json(message)),
            _ => panic!("Unhandled return status: {}", status),
        }
    }

    fn convert_diesel_error(err: DieselError) -> Self {
        let message = ErrorMessage::from_str(&err.to_string());
        match err {
            DieselError::NotFound => Self::NotFound(Json(message)),
            DieselError::DatabaseError(error_kind, _) => match error_kind {
                DatabaseErrorKind::NotNullViolation => Self::BadRequest(Json(message)),
                DatabaseErrorKind::UniqueViolation => Self::Conflict(Json(message)),
                _ => Self::InternalServerError(Json(message)),
            },
            _ => Self::InternalServerError(Json(message)),
        }
    }

    fn convert_serde_error(err: SerdeError<'_>) -> Self {
        let error_value: Value = serde_json::from_str(&err.to_string())
            .unwrap_or_else(|_| Value::String(err.to_string()));
        let message = ErrorMessage::from_value(error_value);
        Self::UnprocessableEntity(Json(message))
    }

    fn convert_serde_valid_error(err: SerdeValidErrors) -> Self {
        let error_value: Value = serde_json::from_str(&err.to_string())
            .unwrap_or_else(|_| Value::String(err.to_string()));
        let message = ErrorMessage::from_value(error_value);
        Self::UnprocessableEntity(Json(message))
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

impl From<SerdeValidErrors> for ErrorResponse {
    fn from(error: SerdeValidErrors) -> ErrorResponse {
        Self::convert_serde_valid_error(error)
    }
}
