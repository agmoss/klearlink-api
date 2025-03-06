pub mod schema;

use core::{cors::CORS, pool::Db};
use log::info;
use logger::setup_logger::setup_logger;

use rocket::{routes, Build, Rocket};

mod base;
mod consumer_credit;
mod core;
mod error;
mod logger;
mod user;

pub fn create_rocket() -> Rocket<Build> {
    setup_logger().expect("Failed to initialize logger");
    info!("Starting Rocket application...");
    rocket::build()
        .register("/", error::catchers())
        .attach(Db::fairing())
        .attach(CORS)
        .mount(
            "/",
            routes![
                base::routes::base_route,
                base::routes::favicon,
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
