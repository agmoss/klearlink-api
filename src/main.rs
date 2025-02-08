pub mod models;
pub mod schema;

use rocket::{launch, routes, Build, Rocket};

mod auth;
mod conn;
mod dto;
mod error;
mod generic;
mod response;
mod routes;

use auth::AuthStore;

#[cfg(test)]
mod tests;

fn create_rocket() -> Rocket<Build> {
    rocket::build()
        .register("/", error::catchers())
        .manage(AuthStore::new())
        .mount(
            "/",
            routes![
                routes::submit_consumer_credit,
                routes::update_consumer_credit,
                routes::view_consumer_credit,
                routes::view_consumer_match
            ],
        )
}
#[launch]
fn rocket() -> Rocket<Build> {
    create_rocket()
}
