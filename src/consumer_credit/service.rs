use crate::consumer_credit::dto::{ConsumerCreditDto, ConsumerCreditEventsDto, ConsumerMatchDto};
use crate::consumer_credit::models::{ConsumerCreditEventModel, ConsumerCreditModel};
use crate::core::auth::AuthResponse;
use crate::core::execute_db_operation::{execute_db_operation, execute_db_operation_rest};
use crate::core::pool::Db;
use crate::core::response::{validate_dto, BaseResponse, ErrorResponse, RestDto, RestResult};
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde_json::Value;

use super::dto::{InsertConsumerCreditDto, UpdateConsumerCreditDto};

pub struct ConsumerCreditService;

impl ConsumerCreditService {
    pub async fn submit_consumer_credit<'r>(
        _consumer_credit_id: String,
        record: RestDto<'r, InsertConsumerCreditDto>,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerCreditDto> {
        let auth_result = auth?;

        let dto = validate_dto(record)?;

        let result = execute_db_operation(
            &conn,
            move |c| {
                use crate::schema::consumer_credit::dsl::*;
                diesel::insert_into(consumer_credit)
                    .values(
                        dto.to_insert_consumer_credit_model(&_consumer_credit_id, &auth_result.id),
                    )
                    .get_result::<ConsumerCreditModel>(c)
            },
            |target| Ok(target),
        )
        .await?;

        ConsumerCreditService::log_event(
            conn,
            &result.consumer_credit_id,
            "ConsumerCreditCreated",
            serde_json::to_value(&result)?,
        )
        .await?;

        Ok(Json(result.into()))
    }

    pub async fn update_consumer_credit<'r>(
        _consumer_credit_id: String,
        record: RestDto<'r, UpdateConsumerCreditDto>,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerCreditDto> {
        auth?;

        let dto = validate_dto(record)?;

        let result = execute_db_operation(
            &conn,
            move |c| {
                use crate::schema::consumer_credit::dsl::*;

                diesel::update(consumer_credit.filter(consumer_credit_id.eq(&_consumer_credit_id)))
                    .set(dto.to_update_consumer_credit_model(&_consumer_credit_id))
                    .get_result::<ConsumerCreditModel>(c)
            },
            |target| Ok(target),
        )
        .await?;

        ConsumerCreditService::log_event(
            conn,
            &result.consumer_credit_id,
            "ConsumerCreditUpdated",
            serde_json::to_value(&result)?,
        )
        .await?;

        Ok(Json(result.into()))
    }

    pub async fn view_consumer_credit(
        _consumer_credit_id: String,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerCreditDto> {
        let auth_result = auth?;
        let target_record =
            Self::get_target_record(_consumer_credit_id, auth_result.id, &conn).await;
        target_record
            .map(|record| Json(record.into()))
            .map_err(ErrorResponse::from)
    }

    pub async fn view_consumer_match(
        _consumer_credit_id: String,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerMatchDto> {
        let auth_result = auth?;
        let target_record =
            Self::get_target_record(_consumer_credit_id, auth_result.id, &conn).await;

        match target_record {
            Ok(target) => {
                let _target: ConsumerCreditModel = target.clone();

                execute_db_operation_rest(
                    &conn,
                    move |c| {
                        use crate::schema::consumer_credit::dsl::*;

                        consumer_credit
                            .or_filter(first_name.eq(&target.first_name))
                            .or_filter(last_name.eq(&target.last_name))
                            .or_filter(email.eq(&target.email))
                            .or_filter(date_of_birth.eq(&target.date_of_birth))
                            .or_filter(address.eq(&target.address))
                            .or_filter(phone_number.eq(&target.phone_number))
                            .filter(user_id.ne(&auth_result.id))
                            .load::<ConsumerCreditModel>(c)
                    },
                    |matched_records: Vec<ConsumerCreditModel>| {
                        Ok(Json(
                            _target.to_consumer_match_dto(
                                matched_records
                                    .into_iter()
                                    .map(|r| r.to_consumer_matches_dto(&_target))
                                    .collect(),
                            ),
                        ))
                    },
                )
                .await
            }
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    pub async fn delete_consumer_credits_by_username(username: String, conn: Db) -> RestResult<()> {
        let user_id_result = Self::get_user_id_by_username(username, &conn).await?;

        execute_db_operation_rest(
            &conn,
            move |c| {
                use crate::schema::consumer_credit::dsl::*;
                diesel::delete(consumer_credit.filter(user_id.eq(user_id_result))).execute(c)
            },
            |_| Ok(Json(())),
        )
        .await
    }

    async fn get_target_record(
        _consumer_credit_id: String,
        _user_id: i32,
        conn: &Db,
    ) -> BaseResponse<ConsumerCreditModel> {
        execute_db_operation(
            conn,
            move |c| {
                use crate::schema::consumer_credit::dsl::*;
                consumer_credit
                    .filter(consumer_credit_id.eq(_consumer_credit_id))
                    .filter(user_id.eq(_user_id))
                    .first::<ConsumerCreditModel>(c)
            },
            |target| Ok(target),
        )
        .await
    }

    async fn get_user_id_by_username(username: String, conn: &Db) -> BaseResponse<i32> {
        execute_db_operation(
            conn,
            move |c| {
                use crate::schema::users::dsl::{username as user_username, users};
                users
                    .filter(user_username.eq(username))
                    .select(crate::schema::users::dsl::id)
                    .first::<i32>(c)
            },
            |target| Ok(target),
        )
        .await
    }

    async fn log_event(
        conn: Db,
        _consumer_credit_id: &str,
        _event_type: &str,
        _event_data: Value,
    ) -> BaseResponse<()> {
        let event_dto = ConsumerCreditEventsDto {
            consumer_credit_id: _consumer_credit_id.to_string(),
            event_type: _event_type.to_string(),
            event_data: _event_data,
        };

        execute_db_operation(
            &conn,
            move |c| {
                use crate::schema::consumer_credit_events::dsl::*;
                diesel::insert_into(consumer_credit_events)
                    .values(event_dto.to_insert_consumer_credit_events_model())
                    .get_result::<ConsumerCreditEventModel>(c)
            },
            |_| Ok(()),
        )
        .await
    }
}
