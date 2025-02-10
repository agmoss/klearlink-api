use diesel::prelude::*;
use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};
use crate::core::conn::establish_connection_pg;
use crate::user::models::Users;

pub struct AuthStore;

impl AuthStore {
    pub fn validate(username: &str, api_key: &str) -> bool {
        use crate::schema::users::dsl::*;

        let connection = &mut establish_connection_pg();
        users
            .filter(username.eq(username))
            .filter(api_key.eq(api_key))
            .first::<Users>(connection)
            .is_ok()
    }
}

#[derive(Debug)]
#[derive(Debug)]
pub struct ApiKeyAuth {
    pub id: i32,
    pub username: String,
    #[allow(dead_code)]
    pub api_key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_store = req.guard::<&State<AuthStore>>().await.unwrap();

        let username = req.headers().get_one("X-Username");
        let api_key = req.headers().get_one("X-API-Key");

        if let (Some(user), Some(key)) = (username, api_key) {
            use crate::schema::users::dsl::*;
            let connection = &mut establish_connection_pg();

            match users
                .filter(username.eq(user))
                .filter(api_key.eq(key))
                .first::<Users>(connection)
            {
                Ok(user_record) => Outcome::Success(ApiKeyAuth {
                    id: user_record.id,
                    username: user_record.username,
                    api_key: user_record.api_key,
                }),
                Err(_) => Outcome::Failure((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
