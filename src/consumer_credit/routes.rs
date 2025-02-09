use diesel::prelude::*;
use diesel::result::Error;
use rocket::{get, post, put, serde::json::Json};

use super::dto::ConsumerCreditDto;
use super::models::ConsumerCredit;
use crate::consumer_credit::dto::{ConsumerMatchDto, MatchedOnDto};
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

#[get("/consumer-credit/<consumer_credit_id_dto>/consumer-match")]
pub async fn view_consumer_match(
    consumer_credit_id_dto: &str,
    _auth: ApiKeyAuth,
) -> RestResult<Vec<ConsumerMatchDto>> {
    use crate::schema::consumer_credit::dsl::*;

    let connection = &mut establish_connection_pg();

    let target_record = consumer_credit
        .filter(consumer_credit_id.eq(consumer_credit_id_dto))
        .first::<ConsumerCredit>(connection);

    match target_record {
        Ok(target) => {
            let matches: Result<Vec<ConsumerCredit>, Error> = consumer_credit
                .or_filter(first_name.eq(&target.first_name))
                .or_filter(last_name.eq(&target.last_name))
                .or_filter(email.eq(&target.email))
                .or_filter(date_of_birth.eq(&target.date_of_birth))
                .or_filter(address.eq(&target.address))
                .or_filter(phone_number.eq(&target.phone_number))
                .filter(tenant.ne(&_auth.username))
                .load::<ConsumerCredit>(connection);

            match matches {
                Ok(records) => {
                    let matched_records: Vec<ConsumerMatchDto> = records
                        .into_iter()
                        .map(|r| {
                            let matched_on = MatchedOnDto {
                                first_name: r.first_name == target.first_name,
                                last_name: r.last_name == target.last_name,
                                email: r.email == target.email,
                                date_of_birth: r.date_of_birth == target.date_of_birth,
                                address: r.address == target.address,
                                phone_number: r.phone_number == target.phone_number,
                            };
                            let consumer_credit_dto: ConsumerCreditDto = r.into();
                            ConsumerMatchDto {
                                consumer_credit: consumer_credit_dto,
                                matched_on,
                            }
                        })
                        .collect();
                    Ok(Json(matched_records))
                }
                Err(e) => Err(ErrorResponse::from(e)),
            }
        }
        Err(e) => Err(ErrorResponse::from(e)),
    }
}
