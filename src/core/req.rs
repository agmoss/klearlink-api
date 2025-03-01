use rocket::serde::json::{Error as RocketSerdeError, Json};

use serde_valid::Validate;

use super::res::ErrorResponse;

pub type RestDto<'a, T> = Result<Json<T>, RocketSerdeError<'a>>;

pub fn validate_dto<T: Validate>(record: RestDto<T>) -> Result<Json<T>, ErrorResponse> {
    let dto = record.map_err(ErrorResponse::from)?;
    dto.validate().map_err(ErrorResponse::from)?;
    Ok(dto)
}
