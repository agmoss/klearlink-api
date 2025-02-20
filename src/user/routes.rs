use crate::core::pool::Db;
use crate::core::response::{RestDto, RestResult};
use rocket::{get, post};

use super::dto::UserDto;
use super::service::UserService;

#[post("/users", data = "<user_dto>")]
pub async fn create_user<'r>(user_dto: RestDto<'r, UserDto>, conn: Db) -> RestResult<UserDto> {
    UserService::create_user_service(user_dto, conn).await
}

#[get("/users/<username>")]
pub async fn view_user(username: String, conn: Db) -> RestResult<UserDto> {
    UserService::view_user(username, conn).await
}
