use crate::user::models::Users;
use crate::user::models::InsertUsers;
use crate::core::pool::Db;
use diesel::prelude::*;
use rocket::{http::Status, request::FromRequest, request::Outcome, request::Request};

pub struct AuthStore;

impl AuthStore {
    /**
     * For testing
     */
    pub async fn new(conn: Db) -> Self {
        use crate::schema::users::dsl::*;

        let user_count: i64 = conn
            .run(|c| users.count().get_result(c))
            .await
            .unwrap_or(0);

        if user_count == 0 {
            let dummy_user = InsertUsers {
                username: "test_user_1".to_string(),
                api_key: "test_key_1".to_string(),
            };

            diesel::insert_into(users)
                .values(&dummy_user)
                .execute(c)
                .expect("Error inserting dummy user");

            let dummy_user_2 = InsertUsers {
                username: "test_user_2".to_string(),
                api_key: "test_key_2".to_string(),
            };

            diesel::insert_into(users)
                .values(&dummy_user_2)
                .execute(c)
                .expect("Error inserting dummy user");
        }

        AuthStore
    }

    pub async fn get_user(u_name: &str, a_key: &str, conn: Db) -> Option<Users> {
        use crate::schema::users::dsl::*;
        conn.run(move |c| {
            users
                .filter(username.eq(u_name))
                .filter(api_key.eq(a_key))
                .first::<Users>(c)
                .ok()
        })
        .await
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
            let conn = req.guard::<Db>().await.unwrap();
            match AuthStore::get_user(user, key, conn).await {
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
