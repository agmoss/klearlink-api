use crate::consumer_credit::dto::{
    ConsumerCreditDto, ConsumerFactsDto, ConsumerMatchDto, ConsumerMatchesDto, CreditFactsDto,
    MatchedCreditFactsDto, MatchedOnDto,
};
use crate::consumer_credit::models::ConsumerCreditModel;
use crate::core::auth::AuthResponse;
use crate::core::execute_db_operation::execute_db_operation;
use crate::core::pool::Db;
use crate::core::response::{DbError, ErrorResponse, RestDto, RestResult};
use diesel::prelude::*;
use rocket::serde::json::Json;
use serde_valid::Validate;

pub struct ConsumerCreditService;

impl ConsumerCreditService {
    pub async fn submit_consumer_credit<'r>(
        _consumer_credit_id: String,
        record: RestDto<'r, ConsumerCreditDto>,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerCreditDto> {
        use crate::schema::consumer_credit::dsl::*;

        let auth_result = auth?;
        let dto = record.map_err(ErrorResponse::from)?;
        dto.validate().map_err(ErrorResponse::from)?;

        execute_db_operation(
            conn,
            move |c| {
                diesel::insert_into(consumer_credit)
                    .values(dto.to_insert_consumer_credit(&_consumer_credit_id, &auth_result.id))
                    .get_result::<ConsumerCreditModel>(c)
            },
            |ok| Ok(Json(ok.into())),
        )
        .await
        .map_err(ErrorResponse::from)
    }

    pub async fn delete_consumer_credits_by_username(username: String, conn: Db) -> RestResult<()> {
        let user_id_result = Self::get_user_id_by_username(username, &conn).await?;

        execute_db_operation(
            conn,
            move |c| {
                use crate::schema::consumer_credit::dsl::*;
                diesel::delete(consumer_credit.filter(user_id.eq(user_id_result))).execute(c)
            },
            |_| Ok(Json(())),
        )
        .await
    }

    pub async fn update_consumer_credit<'r>(
        _consumer_credit_id: String,
        record: RestDto<'r, ConsumerCreditDto>,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<ConsumerCreditDto> {
        use crate::schema::consumer_credit::dsl::*;

        auth?;
        let dto = record.map_err(ErrorResponse::from)?;
        let updated_consumer_facts = dto.to_update_consumer_credit_model(&_consumer_credit_id);

        execute_db_operation(
            conn,
            move |c| {
                diesel::update(consumer_credit.filter(consumer_credit_id.eq(_consumer_credit_id)))
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
                    .get_result::<ConsumerCreditModel>(c)
            },
            |ok| Ok(Json(ok.into())),
        )
        .await
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
        use crate::schema::consumer_credit::dsl::*;

        let auth_result = auth?;
        let target_record =
            Self::get_target_record(_consumer_credit_id, auth_result.id, &conn).await;

        match target_record {
            Ok(target) => {
                let _target: ConsumerCreditModel =
                    serde_json::from_str(&serde_json::to_string(&target).unwrap()).unwrap();

                execute_db_operation(
                    conn,
                    move |c| {
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
                    |records| {
                        let matched_records: Vec<ConsumerMatchesDto> = records
                            .into_iter()
                            .map(|r| ConsumerMatchesDto {
                                matched_on: MatchedOnDto {
                                    first_name: r.first_name == _target.first_name,
                                    last_name: r.last_name == _target.last_name,
                                    email: r.email == _target.email,
                                    date_of_birth: r.date_of_birth == _target.date_of_birth,
                                    address: r.address == _target.address,
                                    phone_number: r.phone_number == _target.phone_number,
                                },
                                credit_facts: MatchedCreditFactsDto {
                                    amount: r.amount,
                                    credit_type: r.credit_type,
                                    application_datetime: r.application_datetime,
                                    originated_datetime: r.originated_datetime,
                                    payment_due_date: r.payment_due_date,
                                    payment_due_amount: r.payment_due_amount,
                                    credit_state: r.credit_state,
                                    institution_names: r.institution_names,
                                },
                            })
                            .collect();

                        Ok(Json(ConsumerMatchDto {
                            consumer_facts: ConsumerFactsDto {
                                first_name: _target.first_name.clone(),
                                last_name: _target.last_name.clone(),
                                email: _target.email.clone(),
                                date_of_birth: _target.date_of_birth,
                                address: _target.address.clone(),
                                phone_number: _target.phone_number.clone(),
                                sin_ssn: _target.sin_ssn.clone(),
                                institution_names: _target.institution_names.clone(),
                            },
                            credit_facts: CreditFactsDto {
                                amount: _target.amount.clone(),
                                credit_type: _target.credit_type.clone(),
                                application_datetime: _target.application_datetime,
                                originated_datetime: _target.originated_datetime.clone(),
                                payment_due_date: _target.payment_due_date.clone(),
                                payment_due_amount: _target.payment_due_amount.clone(),
                                credit_state: _target.credit_state.clone(),
                            },
                            created_at: _target.created_at,
                            updated_at: _target.updated_at,
                            consumer_match: Some(matched_records),
                        }))
                    },
                )
                .await
            }
            Err(e) => Err(ErrorResponse::from(e)),
        }
    }

    async fn get_target_record(
        _consumer_credit_id: String,
        _user_id: i32,
        conn: &Db,
    ) -> Result<ConsumerCreditModel, DbError> {
        use crate::schema::consumer_credit::dsl::*;

        conn.run(move |c| {
            consumer_credit
                .filter(consumer_credit_id.eq(_consumer_credit_id))
                .filter(user_id.eq(_user_id))
                .first::<ConsumerCreditModel>(c)
        })
        .await
    }

    async fn get_user_id_by_username(username: String, conn: &Db) -> Result<i32, DbError> {
        use crate::schema::users::dsl::{username as user_username, users};

        conn.run(move |c| {
            users
                .filter(user_username.eq(username))
                .select(crate::schema::users::dsl::id)
                .first::<i32>(c)
        })
        .await
    }
}
