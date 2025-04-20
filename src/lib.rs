#![allow(clippy::needless_lifetimes)]
#![allow(clippy::redundant_closure)]

pub mod schema;

use core::{cors::Cors, pool::Db, trace::init_tracing_2};

use rocket::{routes, Build, Rocket};

use tracing::warn;

mod base;
pub mod consumer_credit;
mod core;
pub mod error;
pub mod user;

pub fn create_rocket() -> Rocket<Build> {
    init_tracing_2();

    warn!("Starting Rocket application...");
    rocket::build()
        .register("/", error::catchers())
        .attach(Db::fairing())
        .attach(Cors)
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
