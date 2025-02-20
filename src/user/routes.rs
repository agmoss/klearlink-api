use crate::core::pool::Db;
use crate::core::response::RestResult;
use crate::user::models::{InsertUsers, Users};
use crate::user::service::UserService;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[post("/users", data = "<new_user>")]
pub async fn create_user(new_user: Json<InsertUsers>, conn: Db) -> RestResult<Users> {
    UserService::create_user_service(new_user.into_inner(), conn).await
}

#[get("/users/<user_id>")]
pub async fn get_user(user_id: i32, conn: Db) -> RestResult<Users> {
    UserService::get_user_service(user_id, conn).await
}

#[put("/users/<user_id>", data = "<updated_user>")]
pub async fn update_user(
    user_id: i32,
    updated_user: Json<InsertUsers>,
    conn: Db,
) -> RestResult<Users> {
    UserService::update_user_service(user_id, updated_user.into_inner(), conn).await
}

#[delete("/users/<user_id>")]
pub async fn delete_user(user_id: i32, conn: Db) -> RestResult<()> {
    UserService::delete_user_service(user_id, conn).await
}
