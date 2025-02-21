use crate::core::auth::AuthResponse;
use crate::core::pool::Db;
use crate::core::response::{RestDto, RestResult};
use rocket::{delete, get, post};

use super::dto::UserDto;
use super::service::UserService;

#[post("/users", data = "<user_dto>")]
pub async fn create_user<'r>(
    user_dto: RestDto<'r, UserDto>,
    auth: AuthResponse,
    conn: Db,
) -> RestResult<UserDto> {
    if let Ok(user) = auth {
        if user.role == "admin" {
            UserService::create_user_service(user_dto, conn).await
        } else {
            Err(ErrorResponse::Unauthorized(Json(ErrorMessage::from_str(
                "Access denied: Admin role required",
            ))))
        }
    } else {
        Err(auth.unwrap_err())
    }
}

#[delete("/users/<username>")]
pub async fn delete_user(username: String, auth: AuthResponse, conn: Db) -> RestResult<()> {
    if let Ok(user) = auth {
        if user.role == "admin" {
            UserService::delete_user(username, conn).await
        } else {
            Err(ErrorResponse::Unauthorized(Json(ErrorMessage::from_str(
                "Access denied: Admin role required",
            ))))
        }
    } else {
        Err(auth.unwrap_err())
    }
}

#[get("/users/<username>")]
pub async fn view_user(username: String, auth: AuthResponse, conn: Db) -> RestResult<UserDto> {
    if let Ok(user) = auth {
        if user.role == "admin" {
            UserService::view_user(username, conn).await
        } else {
            Err(ErrorResponse::Unauthorized(Json(ErrorMessage::from_str(
                "Access denied: Admin role required",
            ))))
        }
    } else {
        Err(auth.unwrap_err())
    }
}
