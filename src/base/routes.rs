use std::env;

use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiInfo {
    version: &'static str,
    env: String,
    description: &'static str,
    help_link: &'static str,
}

#[get("/")]
pub fn base_route() -> Json<ApiInfo> {    
    Json(ApiInfo {
        version: env!("CARGO_PKG_VERSION"),
        env: env::var("ROCKET_ENV").unwrap_or_else(|_| "unknown".to_string()),
        description: env!("CARGO_PKG_DESCRIPTION"),
        help_link: "https://klearlink.io/",
    })
}

#[get("/favicon.ico")]
pub fn favicon() -> Status {
    Status::NoContent
}
