use diesel::prelude::*;
use rocket::{get, http::Status, post, put, serde::json::Json};
use serde_json::json;

use super::dto::ConsumerCreditDto;
use super::models::ConsumerCredit;
use crate::core::auth::ApiKeyAuth;
use crate::core::conn::establish_connection_pg;
use crate::core::response::{ErrorResponse, RestDto, RestResult};

#[put("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
pub async fn submit_consumer_credit<'r>(
    consumer_credit_id_dto: &str,
    record: RestDto<'r, ConsumerCreditDto>,
    _auth: ApiKeyAuth,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let dto = match record {
        Ok(valid_record) => valid_record,
        Err(err) => return Err(ErrorResponse::from(err)),
    };

    match diesel::insert_into(consumer_credit)
        .values(dto.to_insert_consumer_credit(consumer_credit_id_dto, &_auth.username.clone()))
        .execute(&mut establish_connection_pg())
    {
        Ok(_) => Ok(dto),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[post("/consumer-credit/<consumer_credit_id_dto>", data = "<record>")]
pub async fn update_consumer_credit<'r>(
    consumer_credit_id_dto: &str,
    record: RestDto<'r, ConsumerCreditDto>,
    _auth: ApiKeyAuth,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let dto = match record {
        Ok(valid_record) => valid_record,
        Err(err) => return Err(ErrorResponse::from(err)),
    };

    let updated_consumer_facts = dto.to_update_consumer_credit_model(consumer_credit_id_dto);

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
        Ok(_ok) => Ok(dto),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

#[get("/consumer-credit/<consumer_credit_id_dto>")]
pub async fn view_consumer_credit(
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
pub async fn view_consumer_match(id: String, _auth: ApiKeyAuth) -> RestResult<Json<Vec<ConsumerCreditDto>>> {
    use crate::schema::consumer_credit::dsl::*;

    let connection = &mut establish_connection_pg();

    // Retrieve the consumer credit record by ID
    let target_record = consumer_credit
        .filter(consumer_credit_id.eq(&id))
        .first::<ConsumerCredit>(connection);

    match target_record {
        Ok(target) => {
            // Find matches based on consumer facts
            let matches = consumer_credit
                .filter(first_name.eq(&target.first_name))
                .filter(last_name.eq(&target.last_name))
                .filter(email.eq(&target.email))
                .filter(date_of_birth.eq(&target.date_of_birth))
                .load::<ConsumerCredit>(connection);

            match matches {
                Ok(records) => {
                    let matched_records: Vec<ConsumerCreditDto> = records.into_iter().map(|r| {
                        let matched_on = json!({
                            "first_name": r.first_name == target.first_name,
                            "last_name": r.last_name == target.last_name,
                            "email": r.email == target.email,
                            "date_of_birth": r.date_of_birth == target.date_of_birth,
                            "address": r.address == target.address,
                            "phone_number": r.phone_number == target.phone_number,
                            "institution_names": r.institution_names == target.institution_names,
                        });

                        let mut dto: ConsumerCreditDto = r.into();
                        dto.matched_on = Some(matched_on);
                        dto
                    }).collect();
                    Ok(Json(matched_records))
                }
                Err(_) => Err(ErrorResponse::InternalServerError("Error retrieving matches.".into())),
            }
        }
        Err(diesel::result::Error::NotFound) => Err(ErrorResponse::NotFound("No consumer credit record found for this ID.".into())),
        Err(_) => Err(ErrorResponse::InternalServerError("Error retrieving consumer credit record.".into())),
    }
}
