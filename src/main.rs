pub mod schema;

use core::pool::Db;
use dotenvy::dotenv;

use rocket::{launch, routes, Build, Rocket};

mod consumer_credit;
mod core;
mod error;
mod user;

#[cfg(test)]
mod tests;

fn create_rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", error::catchers())
        .attach(Db::fairing())
        .mount(
            "/",
            routes![
                consumer_credit::routes::submit_consumer_credit,
                consumer_credit::routes::update_consumer_credit,
                consumer_credit::routes::view_consumer_credit,
                consumer_credit::routes::view_consumer_match,
                user::routes::update_user,
                user::routes::create_user,
                user::routes::delete_user,
                user::routes::get_user,
            ],
        )
}
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    create_rocket()
}
