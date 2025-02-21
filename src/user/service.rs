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

        Self::execute_db_operation(
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
        Self::execute_db_operation(
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
        let target_record = Self::get_target_record(_username, &conn).await;
        target_record
            .map(|record| Json(record.into()))
            .map_err(ErrorResponse::from)
    }

    async fn execute_db_operation<T, F, R>(
        conn: Db,
        db_op: F,
        success_handler: impl Fn(T) -> RestResult<R>,
    ) -> RestResult<R>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
        T: Send + 'static,
    {
        let result = conn.run(db_op).await;
        match result {
            Ok(value) => success_handler(value),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    async fn get_target_record(_username: String, conn: &Db) -> Result<UserModel, DbError> {
        use crate::schema::users::dsl::*;

        conn.run(move |c| users.filter(username.eq(_username)).first::<UserModel>(c))
            .await
    }
}
