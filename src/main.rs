pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use models::ConsumerCreditRecord;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes, Request};

mod auth; // Import the `auth.rs` module

use auth::{ApiKeyAuth, AuthStore};
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
    _auth: ApiKeyAuth,
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
    _auth: ApiKeyAuth,
) -> Status {
    use crate::schema::consumer_facts::dsl::*;
    use crate::schema::credit_facts::dsl::*;

    use models::ConsumerFacts;
    use models::CreditFacts;

    let mut connection = establish_connection_pg();

    // Find the existing consumer facts by id
    let target_consumer = consumer_facts.filter(id.eq(&id));
    let target_credit = credit_facts.filter(consumer_id.eq(&id));

    // Update consumer facts
    let updated_consumer_facts = ConsumerFacts {
        first_name: record.consumer_facts.first_name.clone(),
        last_name: record.consumer_facts.last_name.clone(),
        email: record.consumer_facts.email.clone(),
        date_of_birth: record.consumer_facts.date_of_birth.clone(),
        address: record.consumer_facts.address.clone(),
        phone_number: record.consumer_facts.phone_number.clone(),
        consumer_state: record.consumer_facts.consumer_state.clone(),
        institution_names: record.consumer_facts.institution_names.clone(),
    };

    let consumer_update_result = diesel::update(target_consumer)
        .set(&updated_consumer_facts)
        .execute(&mut connection);

    // Update credit facts
    let updated_credit_facts = CreditFacts {
        amount: record.credit_facts.amount,
        credit_type: record.credit_facts.credit_type.clone(),
        application_datetime: record.credit_facts.application_datetime.clone(),
        credit_state: record.credit_facts.credit_state.clone(),
    };

    let credit_update_result = diesel::update(target_credit)
        .set(&updated_credit_facts)
        .execute(&mut connection);

    match (consumer_update_result, credit_update_result) {
        (Ok(1), Ok(1)) => Status::Ok,
        _ => Status::NotFound,
    }
}

#[get("/consumer-credit/<id>")]
async fn view_consumer_credit(id: String, _auth: ApiKeyAuth) -> Status {
    // Implement logic to retrieve a consumer credit record
    // Return 200 OK or 404 Not Found
    Status::Ok
}

#[get("/consumer-credit/<id>/consumer-match")]
async fn view_consumer_match(id: String, _auth: ApiKeyAuth) -> Status {
    // Implement logic to calculate and return consumer match
    // Return 200 OK or 404 Not Found
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(AuthStore::new()).mount(
        "/",
        routes![
            submit_consumer_credit,
            update_consumer_credit,
            view_consumer_credit,
            view_consumer_match
        ],
    )
}
