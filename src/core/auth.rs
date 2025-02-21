use crate::core::pool::Db;
use crate::user::models::UserModel;
use diesel::prelude::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
};
use uuid::Uuid;

use super::response::{ErrorMessage, ErrorResponse};

pub type AuthResponse = Result<UserModel, ErrorResponse>;

pub struct AuthStore;

impl AuthStore {
    pub async fn get_user(u_name: String, a_key: Uuid, conn: Db) -> Option<UserModel> {
        use crate::schema::users::dsl::*;
        conn.run(move |c| {
            users
                .filter(username.eq(u_name))
                .filter(api_key.eq(a_key))
                .first::<UserModel>(c)
                .ok()
        })
        .await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserModel {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let username = req.headers().get_one("X-Username");
        let api_key = req.headers().get_one("X-API-Key");

        if let (Some(user), Some(key)) = (username, api_key) {
            let conn = req.guard::<Db>().await.unwrap();

            match Uuid::try_parse(key) {
                Ok(okk) => match AuthStore::get_user(user.to_string(), okk, conn).await {
                    Some(user_record) => Outcome::Success(UserModel {
                        id: user_record.id,
                        username: user_record.username,
                        api_key: user_record.api_key,
                        role: user_record.role,
                    }),
                    None => Outcome::Error((
                        Status::UnprocessableEntity,
                        ErrorResponse::NotFound(Json(ErrorMessage::from_str(&format!(
                            "User '{}' not found",
                            user
                        )))),
                    )),
                },
                Err(e) => Outcome::Error((Status::UnprocessableEntity, ErrorResponse::from(e))),
            }
        } else {
            Outcome::Error((
                Status::UnprocessableEntity,
                ErrorResponse::NotFound(Json(ErrorMessage::from_str(
                    "Missing authentication headers",
                ))),
            ))
        }
    }
}
