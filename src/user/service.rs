use crate::core::pool::Db;
use crate::core::response::{DbError, ErrorResponse, RestDto, RestResult};
use crate::user::models::UserModel;
use diesel::prelude::*;
use rocket::serde::json::Json;
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

        let result = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(&InsertUserModel {
                        username: dto.username.clone(),
                        api_key: dto.api_key,
                        role: dto.role.clone(),
                    })
                    .get_result::<UserModel>(c)
            })
            .await;

        match result {
            Ok(user) => Ok(Json(user.into())),
            // Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => {
            //     Err(ErrorResponse::from("User with this username or API key already exists"))
            // }
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    pub async fn view_user(_username: String, conn: Db) -> RestResult<UserDto> {
        let target_record = Self::get_target_record(_username, &conn).await;

        match target_record {
            Ok(record) => Ok(Json(record.into())),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    async fn get_target_record(_username: String, conn: &Db) -> Result<UserModel, DbError> {
        use crate::schema::users::dsl::*;

        conn.run(move |c| users.filter(username.eq(_username)).first::<UserModel>(c))
            .await
    }
}
