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

        let dto = match new_user {
            Ok(valid_record) => valid_record,
            Err(err) => return Err(ErrorResponse::from(err)),
        };

        dto.validate().map_err(ErrorResponse::from)?;

        let adsf = dto.clone();

        let res = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(dto.to_insert_user())
                    .execute(c)
            })
            .await;

        match res {
            Ok(_) => Ok(adsf),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }
}
