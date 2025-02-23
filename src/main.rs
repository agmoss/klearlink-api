pub mod schema;

use core::{cors::CORS, pool::Db};
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
        .attach(CORS)
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
