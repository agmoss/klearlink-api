pub mod schema;

use core::{cors::CORS, pool::Db};
use dotenvy::dotenv;

use rocket::{launch, routes, Build, Rocket, get};
use rocket::serde::json::Json;
use serde::Serialize;

mod consumer_credit;
mod core;
mod error;
mod user;

#[derive(Serialize)]
struct ApiInfo {
    version: &'static str,
    description: &'static str,
    help_link: &'static str,
}

#[get("/")]
fn base_route() -> Json<ApiInfo> {
    Json(ApiInfo {
        version: env!("CARGO_PKG_VERSION"),
        description: env!("CARGO_PKG_DESCRIPTION"),
        help_link: "https://klearlink.io/help",
    })
}
mod tests;

fn create_rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", error::catchers())
        .attach(Db::fairing())
        .attach(CORS)
        .mount("/", routes![base_route])
        .mount(
            "/",
            routes![
                consumer_credit::routes::submit_consumer_credit,
                consumer_credit::routes::update_consumer_credit,
                consumer_credit::routes::view_consumer_credit,
                consumer_credit::routes::view_consumer_match,
                consumer_credit::routes::delete_consumer_credits_by_username,
                user::routes::create_user,
                user::routes::view_user,
                user::routes::delete_user,
            ],
        )
}
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    create_rocket()
}
