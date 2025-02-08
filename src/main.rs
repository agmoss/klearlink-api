pub mod models;
pub mod schema;

use diesel::prelude::*;

use response::{ErrorResponse, RestResult};
use rocket::http::Status;

use rocket::serde::json::Json;
use rocket::{get, launch, post, put, routes, Build, Rocket};

mod auth;

mod dto;

use dto::ConsumerCreditDto;
use models::ConsumerCredit;

use auth::{ApiKeyAuth, AuthStore};

mod conn;

use conn::establish_connection_pg;

mod generic;
mod response;

#[cfg(test)]
mod tests;

#[put("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
fn submit_consumer_credit(
    consumer_credit_id_dto: &str,
    record: Result<Json<ConsumerCreditDto>, rocket::serde::json::Error>,
    _auth: ApiKeyAuth,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    // Handle JSON validation
    let record = match record {
        Ok(valid_record) => valid_record,
        Err(err) => return Err(ErrorResponse::from(err)),
    };

    let conn = &mut establish_connection_pg();

    // Insert into database
    match diesel::insert_into(consumer_credit)
        .values(record.to_insert_consumer_credit(consumer_credit_id_dto, &_auth.username.clone()))
        .execute(conn)
    {
        Ok(_) => Ok(record),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[post("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
async fn update_consumer_credit(
    consumer_credit_id_dto: &str,
    record: Json<ConsumerCreditDto>,
    _auth: ApiKeyAuth,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let updated_consumer_facts = record.to_update_consumer_credit_model(consumer_credit_id_dto);

    let consumer_update_result =
        diesel::update(consumer_credit.filter(consumer_credit_id.eq(consumer_credit_id_dto)))
            .set((
                first_name.eq(updated_consumer_facts.first_name),
                last_name.eq(updated_consumer_facts.last_name),
                email.eq(updated_consumer_facts.email),
                date_of_birth.eq(updated_consumer_facts.date_of_birth),
                address.eq(updated_consumer_facts.address),
                phone_number.eq(updated_consumer_facts.phone_number),
                institution_names.eq(updated_consumer_facts.institution_names),
                amount.eq(updated_consumer_facts.amount),
                credit_type.eq(updated_consumer_facts.credit_type),
                application_datetime.eq(updated_consumer_facts.application_datetime),
                credit_state.eq(updated_consumer_facts.credit_state),
            ))
            .execute(&mut establish_connection_pg());

    match consumer_update_result {
        Ok(_ok) => Ok(record),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[get("/consumer-credit/<consumer_credit_id_dto>")]
async fn view_consumer_credit(
    consumer_credit_id_dto: &str,
    _auth: ApiKeyAuth,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    match consumer_credit
        .filter(consumer_credit_id.eq(consumer_credit_id_dto))
        .first::<ConsumerCredit>(&mut establish_connection_pg())
    {
        Ok(record) => {
            let consumer_credit_record: ConsumerCreditDto = record.into();
            Ok(Json(consumer_credit_record))
        }
        Err(e) => Err(ErrorResponse::from(e)),
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
