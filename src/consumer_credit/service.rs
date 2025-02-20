use crate::consumer_credit::dto::{ConsumerCreditDto, ConsumerMatchDto, MatchedOnDto};
use crate::consumer_credit::models::ConsumerCredit;
use crate::core::pool::Db;
use crate::core::response::{ErrorResponse, RestDto, RestResult};
use diesel::prelude::*;
use diesel::result::Error;
use rocket::serde::json::Json;
use serde_valid::Validate;

pub async fn submit_consumer_credit_service<'r>(
    consumer_credit_id_dto: String,
    record: RestDto<'r, ConsumerCreditDto>,
    usr_id: i32,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let dto = match record {
        Ok(valid_record) => valid_record,
        Err(err) => return Err(ErrorResponse::from(err)),
    };

    dto.validate().map_err(ErrorResponse::from)?;

    let adsf = dto.clone();

    let res = conn
        .run(move |c| {
            diesel::insert_into(consumer_credit)
                .values(dto.to_insert_consumer_credit(&consumer_credit_id_dto, &usr_id))
                .execute(c)
        })
        .await;

    match res {
        Ok(_) => Ok(adsf),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

pub async fn update_consumer_credit_service<'r>(
    consumer_credit_id_dto: String,
    record: RestDto<'r, ConsumerCreditDto>,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let dto = match record {
        Ok(valid_record) => valid_record,
        Err(err) => return Err(ErrorResponse::from(err)),
    };

    let updated_consumer_facts = dto.to_update_consumer_credit_model(&consumer_credit_id_dto);

    let res = conn
        .run(move |c| {
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
                .execute(c)
        })
        .await;

    match res {
        Ok(_ok) => Ok(dto),
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

pub async fn view_consumer_credit_service(
    consumer_credit_id_dto: String,
    conn: Db,
) -> RestResult<ConsumerCreditDto> {
    use crate::schema::consumer_credit::dsl::*;

    let res = conn
        .run(move |c| {
            consumer_credit
                .filter(consumer_credit_id.eq(consumer_credit_id_dto))
                .first::<ConsumerCredit>(c)
        })
        .await;

    match res {
        Ok(record) => {
            let consumer_credit_record: ConsumerCreditDto = record.into();
            Ok(Json(consumer_credit_record))
        }
        Err(e) => Err(ErrorResponse::from(e)),
    }
}

pub async fn view_consumer_match_service(
    consumer_credit_id_dto: String,
    usr_id: i32,
    conn: Db,
) -> RestResult<Vec<ConsumerMatchDto>> {
    use crate::schema::consumer_credit::dsl::*;

    let target_record = conn
        .run(move |c| {
            consumer_credit
                .filter(consumer_credit_id.eq(consumer_credit_id_dto))
                .filter(user_id.eq(usr_id))
                .first::<ConsumerCredit>(c)
        })
        .await;

    match target_record {
        Ok(target) => {
            let copied: ConsumerCredit =
                serde_json::from_str(&serde_json::to_string(&target).unwrap()).unwrap();

            let matches: Result<Vec<ConsumerCredit>, Error> = conn
                .run(move |c| {
                    consumer_credit
                        .or_filter(first_name.eq(&target.first_name))
                        .or_filter(last_name.eq(&target.last_name))
                        .or_filter(email.eq(&target.email))
                        .or_filter(date_of_birth.eq(&target.date_of_birth))
                        .or_filter(address.eq(&target.address))
                        .or_filter(phone_number.eq(&target.phone_number))
                        .filter(user_id.ne(&user_id))
                        .load::<ConsumerCredit>(c)
                })
                .await;

            match matches {
                Ok(records) => {
                    let matched_records: Vec<ConsumerMatchDto> = records
                        .into_iter()
                        .map(|r| {
                            let matched_on = MatchedOnDto {
                                first_name: r.first_name == copied.first_name,
                                last_name: r.last_name == copied.last_name,
                                email: r.email == copied.email,
                                date_of_birth: r.date_of_birth == copied.date_of_birth,
                                address: r.address == copied.address,
                                phone_number: r.phone_number == copied.phone_number,
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
