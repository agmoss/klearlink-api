use diesel::prelude::*;
use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};
use crate::core::conn::establish_connection_pg;
use crate::user::models::Users;

pub struct AuthStore;

impl AuthStore {
    pub fn new() -> Self {
        use crate::schema::users::dsl::*;

        let connection = &mut establish_connection_pg();
        let user_count: i64 = users.count().get_result(connection).unwrap_or(0);

        if user_count == 0 {
            let dummy_user = Users {
                id: 1,
                username: "test_user".to_string(),
                api_key: "test_key".to_string(),
            };

            diesel::insert_into(users)
                .values(&dummy_user)
                .execute(connection)
                .expect("Error inserting dummy user");
        }

        AuthStore
    }
        use crate::schema::users::dsl::*;

        let connection = &mut establish_connection_pg();
        users
            .filter(username.eq(username))
            .filter(api_key.eq(api_key))
            .first::<Users>(connection)
            .ok()
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

        let username = req.headers().get_one("X-Username");
        let api_key = req.headers().get_one("X-API-Key");

        if let (Some(user), Some(key)) = (username, api_key) {
            match AuthStore::get_user(user, key) {
                Some(user_record) => Outcome::Success(ApiKeyAuth {
                    id: user_record.id,
                    username: user_record.username,
                    api_key: user_record.api_key,
                }),
                None => Outcome::Failure((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
