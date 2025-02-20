use crate::core::pool::Db;
use crate::core::response::{RestDto, RestResult};
use rocket::post;

use super::dto::UserDto;
use super::service::UserService;

#[post("/users", data = "<new_user>")]
pub async fn create_user<'r>(new_user: RestDto<'r, UserDto>, conn: Db) -> RestResult<UserDto> {
    UserService::create_user_service(new_user, conn).await
}
