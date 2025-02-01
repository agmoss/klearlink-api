pub mod models;
pub mod schema;

use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::{status::Created, Debug};
use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes, Build, Rocket};

mod auth;

mod dto;

use dto::ConsumerCreditRecord;
use models::ConsumerCredit;

use auth::{ApiKeyAuth, AuthStore};

mod conn;

use conn::establish_connection_pg;

#[cfg(test)]
mod tests;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[put("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
async fn submit_consumer_credit(
    consumer_credit_id_dto: &str,
    record: Json<ConsumerCreditRecord>,
    _auth: ApiKeyAuth,
) -> Result<Created<Json<ConsumerCreditRecord>>> {
    use crate::schema::consumer_credit::dsl::*;

    diesel::insert_into(consumer_credit)
        .values(record.to_new_consumer_credit(consumer_credit_id_dto, &_auth.username.clone()))
        .execute(&mut establish_connection_pg())
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

    let updated_consumer_facts = record.to_new_consumer_credit2(consumer_credit_id_dto);

    let consumer_update_result =
        diesel::update(consumer_credit.filter(consumer_credit_id.eq(consumer_credit_id_dto)))
            .set((
                first_name.eq(updated_consumer_facts.first_name),
                last_name.eq(updated_consumer_facts.last_name),
                email.eq(updated_consumer_facts.email),
                date_of_birth.eq(updated_consumer_facts.date_of_birth),
                address.eq(updated_consumer_facts.address),
                phone_number.eq(updated_consumer_facts.phone_number),
                consumer_state.eq(updated_consumer_facts.consumer_state),
                institution_names.eq(updated_consumer_facts.institution_names),
                amount.eq(updated_consumer_facts.amount),
                credit_type.eq(updated_consumer_facts.credit_type),
                application_datetime.eq(updated_consumer_facts.application_datetime),
                credit_state.eq(updated_consumer_facts.credit_state),
            ))
            .execute(&mut establish_connection_pg());

    match consumer_update_result {
        Ok(1) => Status::Ok,
        _ => Status::NotFound,
    }
}

#[get("/consumer-credit/<consumer_credit_id_dto>")]
async fn view_consumer_credit(
    consumer_credit_id_dto: &str,
    _auth: ApiKeyAuth,
) -> Result<Json<ConsumerCreditRecord>, Status> {
    use crate::schema::consumer_credit::dsl::*;

    match consumer_credit
        .filter(consumer_credit_id.eq(consumer_credit_id_dto))
        .first::<ConsumerCredit>(&mut establish_connection_pg())
    {
        Ok(record) => {
            let consumer_credit_record: ConsumerCreditRecord = record.into();
            Ok(Json(consumer_credit_record))
        }
        Err(diesel::result::Error::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/consumer-credit/<id>/consumer-match")]
async fn view_consumer_match(id: String, _auth: ApiKeyAuth) -> Status {
    // Implement logic to calculate and return consumer match
    // Return 200 OK or 404 Not Found
    Status::Ok
}

fn create_rocket() -> Rocket<Build> {
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

#[launch]
fn rocket() -> Rocket<Build> {
    create_rocket()
}
