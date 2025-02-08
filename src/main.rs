pub mod schema;

use core::auth::AuthStore;

use rocket::{launch, routes, Build, Rocket};

mod consumer_credit;
mod core;
mod error;

#[cfg(test)]
mod tests;

fn create_rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", error::catchers())
        .manage(AuthStore::new())
        .mount(
            "/",
            routes![
                consumer_credit::routes::submit_consumer_credit,
                consumer_credit::routes::update_consumer_credit,
                consumer_credit::routes::view_consumer_credit,
                consumer_credit::routes::view_consumer_match
            ],
        )
}
#[launch]
fn rocket() -> Rocket<Build> {
    create_rocket()
}
