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
pub struct ApiKeyAuth {
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

        match (username, api_key) {
            (Some(user), Some(key)) if auth_store.validate(user, key) => {
                Outcome::Success(ApiKeyAuth {
                    username: user.to_string(),
                    api_key: key.to_string(),
                })
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
