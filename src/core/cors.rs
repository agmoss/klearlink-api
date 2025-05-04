use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};
use std::env;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Secure CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Get the environment
        let env = env::var("ROCKET_ENV").unwrap_or_else(|_| "unknown".to_string());

        // Define allowed origins based on environment
        let allowed_origins = if env == "staging" {
            vec!["https://api.klearlink.io", "http://localhost:3000"]
        } else {
            vec!["https://api.klearlink.io"]
        };

        // Get the origin from the request
        let origin = request.headers().get_one("Origin").unwrap_or("");

        // Check if the origin is allowed
        let is_allowed = allowed_origins.contains(&origin);

        // Only set CORS headers for specific routes/methods
        if (request.method() == Method::Options
            || request.method() == Method::Post
            || request.method() == Method::Get)
            && is_allowed
        {
            // Allow the specific origin
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));

            // Explicitly allow only necessary methods
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, PATCH, OPTIONS",
            ));

            // Specify allowed headers explicitly
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization, X-Requested-With",
            ));

            // Allow credentials securely (avoid "*" for origin when using credentials)
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

            // Set max age for preflight requests to reduce the number of OPTIONS requests
            response.set_header(Header::new("Access-Control-Max-Age", "86400"));
        }

        // Handle preflight requests
        if request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
}
