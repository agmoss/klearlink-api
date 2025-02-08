use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct AuthStore {
    users: Mutex<HashMap<String, String>>, // username -> api_key
}

impl AuthStore {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        // fake data
        users.insert("test_user".to_string(), "test_key".to_string());
        Self {
            users: Mutex::new(users),
        }
    }

    pub fn validate(&self, username: &str, api_key: &str) -> bool {
        let users = self.users.lock().unwrap();
        users.get(username).is_some_and(|key| key == api_key)
    }
}

#[derive(Debug)]
pub struct ApiKeyAuth {
    pub username: String,
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
