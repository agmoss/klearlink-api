use crate::core::pool::Db;
use crate::user::models::UserModel;
use diesel::prelude::*;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::reqres::{BaseResponse, ErrorMessage, ErrorResponse};

pub type AuthResponse = BaseResponse<AuthDto>;

#[derive(Deserialize, Serialize)]
pub struct AuthDto {
    /// user id
    pub id: i32,
    pub username: String,
    pub api_key: Uuid,
    pub role: String,
}

impl From<UserModel> for AuthDto {
    fn from(user_model: UserModel) -> Self {
        AuthDto {
            id: user_model.id,
            username: user_model.username,
            api_key: user_model.api_key,
            role: user_model.role,
        }
    }
}

impl AuthDto {
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

    pub fn ensure_admin(&self) -> BaseResponse<()> {
        if self.role == "admin" {
            Ok(())
        } else {
            Err(ErrorResponse::Unauthorized(Json(ErrorMessage::from_str(
                "Access denied: Admin role required",
            ))))
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthDto {
    type Error = ErrorResponse;

    /// Extract Auth username and API_KEY from the "Authorization" header.
    ///
    /// Handlers with AuthResponse guard will fail with 401, 404, or 422 error.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let username = req.headers().get_one("X-Username");
        let api_key = req.headers().get_one("X-API-Key");

        if let (Some(_username), Some(_api_key)) = (username, api_key) {
            let conn = req.guard::<Db>().await.unwrap();

            match Uuid::try_parse(_api_key) {
                Ok(ok_api_key) => {
                    match AuthDto::get_user(_username.to_string(), ok_api_key, conn).await {
                        Some(user_record) => Outcome::Success(user_record.into()),
                        None => Outcome::Error((
                            Status::NotFound,
                            ErrorResponse::NotFound(Json(ErrorMessage::from_str(&format!(
                                "User with credentials '{}' '{}' not found",
                                _username, ok_api_key
                            )))),
                        )),
                    }
                }
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
