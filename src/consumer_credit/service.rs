use crate::consumer_credit::dto::{ConsumerCreditDto, ConsumerMatchDto, MatchedOnDto};
use crate::consumer_credit::models::ConsumerCreditModel;
use crate::core::auth::AuthResponse;
use crate::core::pool::Db;
use crate::core::response::{DbError, ErrorResponse, RestDto, RestResult};
use crate::user::models::UserModel;
use diesel::prelude::*;
use diesel::result::Error;
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

        Self::execute_db_operation(
            conn,
            move |c| {
                diesel::insert_into(consumer_credit)
                    .values(dto.to_insert_consumer_credit(&_consumer_credit_id, &auth_result.id))
                    .get_result::<ConsumerCreditModel>(c)
            },
            |ok| Ok(Json(ok.into())),
        )
        .await
    }

    pub async fn delete_consumer_credits_by_username(username: String, conn: Db) -> RestResult<()> {
        let user_id_result = Self::get_user_id_by_username(username, &conn).await;

        match user_id_result {
            Ok(_user_id) => {
                Self::execute_db_operation(
                    conn,
                    move |c| {
                        diesel::delete(consumer_credit.filter(user_id.eq(_user_id))).execute(c)
                    },
                    |_| Ok(Json(())),
                )
                .await
            }
            Err(e) => Err(ErrorResponse::from(e)),
        }
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

        Self::execute_db_operation(
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
        let target_record = Self::get_target_record(_consumer_credit_id, auth_result.id, &conn).await;
        target_record.map(|record| Json(record.into())).map_err(ErrorResponse::from)
    }

    pub async fn view_consumer_match(
        _consumer_credit_id: String,
        auth: AuthResponse,
        conn: Db,
    ) -> RestResult<Vec<ConsumerMatchDto>> {
        use crate::schema::consumer_credit::dsl::*;

        let auth_result = auth?;
        let target_record = Self::get_target_record(_consumer_credit_id, auth_result.id, &conn).await;

        match target_record {
            Ok(target) => {
                let copied: ConsumerCreditModel =
                    serde_json::from_str(&serde_json::to_string(&target).unwrap()).unwrap();

                Self::execute_db_operation(
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
    
    async fn execute_db_operation<T, F, R>(
        conn: Db,
        db_op: F,
        success_handler: impl Fn(T) -> RestResult<R>,
    ) -> RestResult<R>
    where
        F: FnOnce(&mut diesel::PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    {
        let result = conn.run(db_op).await;
        match result {
            Ok(value) => success_handler(value),
            Err(e) => Err(ErrorResponse::from(e)),
        }
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
