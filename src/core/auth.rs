use crate::user::models::Users;
use crate::{core::conn::establish_connection_pg, user::models::InsertUsers};
use diesel::prelude::*;
use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};

pub struct AuthStore;

impl AuthStore {
    /**
     * For testing
     */
    pub fn new() -> Self {
        use crate::schema::users::dsl::*;

        let connection = &mut establish_connection_pg();
        let user_count: i64 = users.count().get_result(connection).unwrap_or(0);

        if user_count == 0 {
            let dummy_user = InsertUsers {
                username: "test_user_1".to_string(),
                api_key: "test_key_1".to_string(),
            };

            diesel::insert_into(users)
                .values(&dummy_user)
                .execute(connection)
                .expect("Error inserting dummy user");

            let dummy_user_2 = InsertUsers {
                username: "test_user_2".to_string(),
                api_key: "test_key_2".to_string(),
            };

            diesel::insert_into(users)
                .values(&dummy_user_2)
                .execute(connection)
                .expect("Error inserting dummy user");
        }

        AuthStore
    }

    pub fn get_user(u_name: &str, a_key: &str) -> Option<Users> {
        use crate::schema::users::dsl::*;
        users
            .filter(username.eq(u_name))
            .filter(api_key.eq(a_key))
            .first::<Users>(&mut establish_connection_pg())
            .ok()
    }
}

#[derive(Debug)]
pub struct ApiKeyAuth {
    pub user_id: i32,
    #[allow(dead_code)]
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
                    user_id: user_record.id,
                    username: user_record.username,
                    api_key: user_record.api_key,
                }),
                None => Outcome::Error((Status::Unauthorized, ())),
            }
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}
