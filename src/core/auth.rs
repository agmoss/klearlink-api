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

use super::res::{BaseResponse, ErrorMessage, ErrorResponse};

pub type AuthResponse = BaseResponse<AuthDto>;

#[derive(Deserialize, Serialize)]
pub struct AuthDto {
    /// user id
    pub id: i32,
    pub api_key: Uuid,
    pub role: String,
}

impl From<UserModel> for AuthDto {
    fn from(user_model: UserModel) -> Self {
        AuthDto {
            id: user_model.id,
            api_key: user_model.api_key,
            role: user_model.role,
        }
    }
}

impl AuthDto {
    pub async fn get_user(a_key: Uuid, conn: Db) -> Option<UserModel> {
        use crate::schema::users::dsl::*;
        conn.run(move |c| users.filter(api_key.eq(a_key)).first::<UserModel>(c).ok())
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

    /// Extract API key from the "Authorization" header.
    ///
    /// Handlers with AuthResponse guard will fail with 401, 404, or 422 error.
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization = req.headers().get_one("Authorization");

        if authorization.is_none() {
            return Outcome::Error((
                Status::UnprocessableEntity,
                ErrorResponse::NotFound(Json(ErrorMessage::from_str(
                    "Missing authentication header",
                ))),
            ));
        }

        let auth_header = authorization.unwrap();

        // Ensure it follows the format "Apikey <key>"
        let parts: Vec<&str> = auth_header.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Apikey" {
            return Outcome::Error((
                Status::BadRequest,
                ErrorResponse::NotFound(Json(ErrorMessage::from_str(
                    "Invalid Authorization format. Expected: 'Authorization: Apikey <UUID>'",
                ))),
            ));
        }

        let api_key_str = parts[1];

        match Uuid::parse_str(api_key_str) {
            Ok(parsed_api_key) => {
                let conn = req.guard::<Db>().await.unwrap();
                match AuthDto::get_user(parsed_api_key, conn).await {
                    Some(user_record) => Outcome::Success(user_record.into()),
                    None => Outcome::Error((
                        Status::NotFound,
                        ErrorResponse::NotFound(Json(ErrorMessage::from_str(&format!(
                            "User with API key '{}' not found",
                            parsed_api_key
                        )))),
                    )),
                }
            }
            Err(_) => Outcome::Error((
                Status::UnprocessableEntity,
                ErrorResponse::NotFound(Json(ErrorMessage::from_str(
                    "Invalid API key format. Expected a valid UUID.",
                ))),
            )),
        }
    }
}
