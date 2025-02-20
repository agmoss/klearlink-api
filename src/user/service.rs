use crate::core::pool::Db;
use crate::core::response::{ErrorResponse, RestDto, RestResult};
use diesel::prelude::*;
use serde_valid::Validate;

use super::dto::UserDto;

pub struct UserService;

impl UserService {
    pub async fn create_user_service<'r>(
        new_user: RestDto<'r, UserDto>,
        conn: Db,
    ) -> RestResult<UserDto> {
        use crate::schema::users::dsl::*;

        let dto = new_user.map_err(ErrorResponse::from)?;

        dto.validate().map_err(ErrorResponse::from)?;

        let insert_user = dto.to_insert_user();

        let result = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(&insert_user)
                    .get_result::<Users>(c)
            })
            .await;

        match result {
            Ok(user) => Ok(Json(user.into())),
            Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
                Err(ErrorResponse::from("User with this username or API key already exists"))
            }
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }
}
