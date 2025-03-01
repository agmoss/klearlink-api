use crate::core::auth::AuthResponse;
use crate::core::pool::Db;
use crate::core::reqres::{RestDto, RestResult};
use rocket::{delete, get, post};

use super::dto::UserDto;
use super::service::UserService;

#[post("/users", data = "<user_dto>")]
pub async fn create_user<'r>(
    user_dto: RestDto<'r, UserDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<UserDto> {
    match auth {
        Ok(user) => {
            user.ensure_admin()?;
            UserService::create_user(user_dto, conn).await
        }
        Err(err) => Err(err),
    }
}

#[delete("/users/<username>")]
pub async fn delete_user(username: String, auth: AuthResponse, conn: Db) -> RestResult<()> {
    match auth {
        Ok(user) => {
            user.ensure_admin()?;
            UserService::delete_user(username, conn).await
        }
        Err(err) => Err(err),
    }
}

#[get("/users/<username>")]
pub async fn view_user(username: String, auth: AuthResponse, conn: Db) -> RestResult<UserDto> {
    match auth {
        Ok(user) => {
            user.ensure_admin()?;
            UserService::view_user(username, conn).await
        }
        Err(err) => Err(err),
    }
}
