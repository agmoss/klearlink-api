use crate::core::pool::Db;
use crate::core::response::{ErrorResponse, RestResult};
use crate::user::models::{InsertUsers, Users};
use diesel::prelude::*;
use rocket::serde::json::Json;

pub struct UserService;

impl UserService {
    pub async fn create_user_service(
        new_user: InsertUsers,
        conn: Db,
    ) -> RestResult<Users> {
        use crate::schema::users::dsl::*;

        let res = conn
            .run(move |c| {
                diesel::insert_into(users)
                    .values(&new_user)
                    .get_result::<Users>(c)
            })
            .await;

        match res {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    pub async fn get_user_service(
        user_id: i32,
        conn: Db,
    ) -> RestResult<Users> {
        use crate::schema::users::dsl::*;

        let res = conn
            .run(move |c| users.filter(id.eq(user_id)).first::<Users>(c))
            .await;

        match res {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    pub async fn update_user_service(
        user_id: i32,
        updated_user: InsertUsers,
        conn: Db,
    ) -> RestResult<Users> {
        use crate::schema::users::dsl::*;

        let res = conn
            .run(move |c| {
                diesel::update(users.filter(id.eq(user_id)))
                    .set((
                        username.eq(updated_user.username),
                        api_key.eq(updated_user.api_key),
                    ))
                    .get_result::<Users>(c)
            })
            .await;

        match res {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    pub async fn delete_user_service(
        user_id: i32,
        conn: Db,
    ) -> RestResult<()> {
        use crate::schema::users::dsl::*;

        let res = conn
            .run(move |c| diesel::delete(users.filter(id.eq(user_id))).execute(c))
            .await;

        match res {
            Ok(_) => Ok(Json(())),
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }
}
