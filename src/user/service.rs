use crate::core::execute_db_operation::execute_db_operation_rest;
use crate::core::pool::Db;
use crate::core::response::{validate_dto, RestDto, RestResult};
use crate::user::models::{InsertUserModel, UserModel};
use diesel::prelude::*;
use rocket::serde::json::Json;

use super::dto::UserDto;

pub struct UserService;

impl UserService {
    pub async fn create_user<'r>(record: RestDto<'r, UserDto>, conn: Db) -> RestResult<UserDto> {
        let dto = validate_dto(record)?;

        execute_db_operation_rest(
            &conn,
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
        execute_db_operation_rest(
            &conn,
            move |c| {
                use crate::schema::users::dsl::*;
                diesel::delete(users.filter(username.eq(_username))).execute(c)
            },
            |_| Ok(Json(())),
        )
        .await
    }

    pub async fn view_user(_username: String, conn: Db) -> RestResult<UserDto> {
        execute_db_operation_rest(
            &conn,
            move |c| {
                use crate::schema::users::dsl::*;
                users.filter(username.eq(_username)).first::<UserModel>(c)
            },
            |user| Ok(Json(user.into())),
        )
        .await
    }
}
