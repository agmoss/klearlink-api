pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::ConsumerCreditRecord;
use rocket::http::Status;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes, Request};
use rocket::request::FromRequest;
use crate::auth::{ApiKey, Username};
use std::env;

#[cfg(test)]
mod tests;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[put("/consumer-credit/<id>", data = "<record>")]
async fn submit_consumer_credit(
    id: String,
    record: Json<ConsumerCreditRecord>,
    _api_key: ApiKey,
    _username: Username,
) -> Result<Created<Json<ConsumerCreditRecord>>> {
    use crate::schema::consumer_facts::dsl::*;
    use crate::schema::credit_facts::dsl::*;

    use models::ConsumerFacts;
    use models::CreditFacts;

    let mut connection = establish_connection_pg();

    // Insert new consumer facts
    let new_consumer_facts = ConsumerFacts {
        first_name: record.consumer_facts.first_name.clone(),
        last_name: record.consumer_facts.last_name.clone(),
        email: record.consumer_facts.email.clone(),
        date_of_birth: record.consumer_facts.date_of_birth.clone(),
        address: record.consumer_facts.address.clone(),
        phone_number: record.consumer_facts.phone_number.clone(),
        consumer_state: record.consumer_facts.consumer_state.clone(),
        institution_names: record.consumer_facts.institution_names.clone(),
    };

    diesel::insert_into(consumer_facts)
        .values(&new_consumer_facts)
        .execute(&mut connection)
        .expect("Error saving new consumer facts");

    // Insert new credit facts
    let new_credit_facts = CreditFacts {
        amount: record.credit_facts.amount,
        credit_type: record.credit_facts.credit_type.clone(),
        application_datetime: record.credit_facts.application_datetime.clone(),
        credit_state: record.credit_facts.credit_state.clone(),
    };

    diesel::insert_into(credit_facts)
        .values(&new_credit_facts)
        .execute(&mut connection)
        .expect("Error saving new credit facts");

    Ok(Created::new("/").body(record))
}

#[post("/consumer-credit/<id>", data = "<record>")]
async fn update_consumer_credit(
    id: String,
    record: Json<ConsumerCreditRecord>,
    _api_key: ApiKey,
    _username: Username,
) -> Status {
    // Implement logic to update an existing consumer credit record
    // Return 200 OK or 404 Not Found
    Status::Ok
}

#[get("/consumer-credit/<id>")]
async fn view_consumer_credit(
    id: String,
    _api_key: ApiKey,
    _username: Username,
) -> Status {
    // Implement logic to retrieve a consumer credit record
    // Return 200 OK or 404 Not Found
    Status::Ok
}

#[get("/consumer-credit/<id>/consumer-match")]
async fn view_consumer_match(
    id: String,
    _api_key: ApiKey,
    _username: Username,
) -> Status {
    // Implement logic to calculate and return consumer match
    // Return 200 OK or 404 Not Found
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            submit_consumer_credit,
            update_consumer_credit,
            view_consumer_credit,
            view_consumer_match
        ],
    )
}
