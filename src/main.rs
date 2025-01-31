pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::http::Status;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes};

mod auth;

mod dto;

use dto::{ConsumerCreditDto, ConsumerCreditRecord};
use models::ConsumerCredit;

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

#[put("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
async fn submit_consumer_credit(
    consumer_credit_id_dto: &str,
    record: Json<ConsumerCreditRecord>,
    _auth: ApiKeyAuth,
) -> Result<Created<Json<ConsumerCreditRecord>>> {
    use crate::schema::consumer_credit::dsl::*;

    let mut connection = establish_connection_pg();

    let new_consumer_facts = record.to_consumer_credit(consumer_credit_id_dto);

    diesel::insert_into(consumer_credit)
        .values(&new_consumer_facts)
        .execute(&mut connection)
        .expect("Error saving new consumer credit");

    Ok(Created::new("/").body(record))
}

#[post("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
async fn update_consumer_credit(
    consumer_credit_id_dto: &str,
    record: Json<ConsumerCreditRecord>,
    _auth: ApiKeyAuth,
) -> Status {
    use crate::schema::consumer_credit::dsl::*;

    let mut connection = establish_connection_pg();

    let target_consumer_credit =
        consumer_credit.filter(consumer_credit_id.eq(consumer_credit_id_dto));

    let updated_consumer_facts = record.to_consumer_credit(consumer_credit_id_dto);

    let consumer_update_result = diesel::update(target_consumer_credit)
        .set((
            first_name.eq(updated_consumer_facts.first_name),
            email.eq(updated_consumer_facts.email),
        ))
        .execute(&mut connection);

    match consumer_update_result {
        Ok(1) => Status::Ok,
        _ => Status::NotFound,
    }
}

#[get("/consumer-credit/<consumer_credit_id_dto>")]
async fn view_consumer_credit(consumer_credit_id_dto: &str, _auth: ApiKeyAuth) -> Result<Json<ConsumerCredit>, Status> {
    use crate::schema::consumer_credit::dsl::*;

    let mut connection = establish_connection_pg();

    match consumer_credit.filter(consumer_credit_id.eq(&consumer_credit_id_dto)).first::<ConsumerCredit>(&mut connection) {
        Ok(record) => Ok(Json(record)),
        Err(_) => Err(Status::NotFound),
    }
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
