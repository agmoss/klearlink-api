use crate::core::execute_db_operation::execute_db_operation;
use crate::core::pool::Db;
use crate::core::response::{DbError, ErrorResponse, RestDto, RestResult};
use crate::user::models::{InsertUserModel, UserModel};
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde_valid::Validate;

use super::dto::UserDto;

pub struct UserService;

impl UserService {
    pub async fn create_user<'r>(new_user: RestDto<'r, UserDto>, conn: Db) -> RestResult<UserDto> {
        let dto = new_user.map_err(ErrorResponse::from)?;
        dto.validate().map_err(ErrorResponse::from)?;

        execute_db_operation(
            conn,
            move |c| {
                use crate::schema::users::dsl::*;
                diesel::insert_into(users)
                    .values(&InsertUserModel::new(
                        dto.username.clone(),
                        dto.api_key,
                        dto.role.clone(),
                    ))
                    .get_result::<UserModel>(c)
            },
            |user| Ok(Json(user.into())),
        )
        .await
    }

    pub async fn delete_user(_username: String, conn: Db) -> RestResult<()> {
        execute_db_operation(
            conn,
            move |c| {
                use crate::schema::users::dsl::*;
                diesel::delete(users.filter(username.eq(_username))).execute(c)
            },
            |_| Ok(Json(())),
        )
        .await
    }

    pub async fn view_user(_username: String, conn: Db) -> RestResult<UserDto> {
        let target_record = Self::get_target_record(_username, &conn).await?;
        Ok(Json(target_record.into()))
    }

    async fn get_target_record(_username: String, conn: &Db) -> Result<UserModel, DbError> {
        use crate::schema::users::dsl::*;

        conn.run(move |c| users.filter(username.eq(_username)).first::<UserModel>(c))
            .await
    }
}
