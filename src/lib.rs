pub mod schema;

use core::{cors::CORS, pool::Db, trace::init_tracing};

use rocket::{routes, Build, Rocket};

use tracing::warn;

mod base;
mod consumer_credit;
mod core;
mod error;
mod user;

pub fn create_rocket() -> Rocket<Build> {
    init_tracing().expect("tracing on");

    warn!("Starting Rocket application...");
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
