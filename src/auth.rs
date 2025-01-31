use rocket::request::{FromRequest, Outcome, Request};
use rocket::http::Status;

pub struct ApiKey(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-API-Key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        Outcome::Success(ApiKey(keys[0].to_string()))
    }
}
