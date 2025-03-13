use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Secure CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Define allowed origin (specific domain instead of wildcard)
        let allowed_origin = "https://api.klearlink.io"; // Change this to your frontend's domain

        // Only set CORS headers for specific routes/methods
        if request.method() == Method::Options
            || request.method() == Method::Post
            || request.method() == Method::Get
        {
            // Allow only specific origin
            response.set_header(Header::new("Access-Control-Allow-Origin", allowed_origin));

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
