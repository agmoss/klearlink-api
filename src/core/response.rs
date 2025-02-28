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
    pub fn from_str(msg: &str) -> Self {
        Self {
            error: Value::String(msg.to_owned()),
        }
    }
    pub fn from_value(value: Value) -> Self {
        Self { error: value }
    }
}

/// Enumeration containing many kinds of error responses to a REST request that
/// was received. All of these responses contain a JSON structure with a root key of "error".
/// These error responses are modelled after HTTP response codes.
#[derive(Responder, Debug, Clone)]
pub enum ErrorResponse {
    /// 400 Bad Request
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorMessage>),

    /// 401 Unauthorized
    #[response(status = 401, content_type = "json")]
    Unauthorized(Json<ErrorMessage>),

    /// 404 Not Found
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorMessage>),

    /// 408 Request Timeout
    #[response(status = 408, content_type = "json")]
    RequestTimeout(Json<ErrorMessage>),

    /// 409 Conflict
    #[response(status = 409, content_type = "json")]
    Conflict(Json<ErrorMessage>),

    /// 412 Precondition Failed
    #[response(status = 412, content_type = "json")]
    PreconditionFailed(Json<ErrorMessage>),

    /// 422 Unprocessable Entity
    #[response(status = 422, content_type = "json")]
    UnprocessableEntity(Json<ErrorMessage>),

    /// 444 No Response
    #[response(status = 444, content_type = "json")]
    NoResponse(Json<ErrorMessage>),

    /// 499 Client Closed Request
    #[response(status = 499, content_type = "json")]
    ClientClosedRequest(Json<ErrorMessage>),

    /// 500 Internal Server Error
    #[response(status = 500, content_type = "json")]
    InternalServerError(Json<ErrorMessage>),

    /// 501 Not Implemented
    #[response(status = 501, content_type = "json")]
    NotImplemented(Json<ErrorMessage>),

    /// 503 Service Unavailable
    #[response(status = 503, content_type = "json")]
    ServiceUnavailable(Json<ErrorMessage>),

    /// 511 Network Authentication Required
    #[response(status = 511, content_type = "json")]
    NetworkAuthenticationRequired(Json<ErrorMessage>),
}

pub type DbError = DieselError;

pub type RestDto<'a, T> = Result<Json<T>, SerdeError<'a>>;

pub type RestResult<T> = Result<Json<T>, ErrorResponse>;

fn get_constraint_name(error: &DieselError) -> Option<&str> {
    if let DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) = error {
        return info.constraint_name();
    }
    if let DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) = error {
        return info.constraint_name();
    }
    if let DieselError::DatabaseError(DatabaseErrorKind::CheckViolation, info) = error {
        return info.constraint_name();
    }
    None
}

impl ErrorResponse {
    /// Actual internal conversion function for generating an error `Response`
    /// from a http `Status`. The `Status` message will be converted into a JSON
    /// object containing a single `"error"` field, which will be the response
    /// body.
    ///
    /// # Panics
    /// This function will panic if using an unhandled status code or if the
    /// status code is "`Ok`", in which case it should have been a successful
    /// response instead.
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

    fn convert_diesel_error(err: DbError) -> Self {
        let message = ErrorMessage::from_str(&err.to_string());
        match err {
            DbError::NotFound => Self::NotFound(Json(message)),
            DbError::DatabaseError(error_kind, _) => match error_kind {
                DatabaseErrorKind::NotNullViolation => Self::BadRequest(Json(message)),
                DatabaseErrorKind::CheckViolation => {
                    let constraint_name = get_constraint_name(&err);
                    Self::Conflict(Json(message))
                },
                DatabaseErrorKind::UniqueViolation => {
                    let constraint_name = get_constraint_name(&err);
                    let custom_message = if constraint_name
                        == Some("consumer_credit_consumer_credit_id_user_id_key")
                    {
                        ErrorMessage::from_str("Duplicate consumer credit record error")
                    } else {
                        message
                    };
                    Self::Conflict(Json(custom_message))
                }
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

    fn convert_uuid_valid_error(err: uuid::Error) -> Self {
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

impl From<DbError> for ErrorResponse {
    fn from(error: DbError) -> ErrorResponse {
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

impl From<uuid::Error> for ErrorResponse {
    fn from(error: uuid::Error) -> ErrorResponse {
        Self::convert_uuid_valid_error(error)
    }
}
