use crate::schema::consumer_credit;
use crate::schema::consumer_credit_events;
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Queryable, Insertable, Clone)]
#[diesel(table_name = consumer_credit)]
pub struct ConsumerCreditModel {
    pub id: i32,
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[diesel(sql_type = Date)]
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub sin_ssn: Option<String>,
    #[diesel(sql_type = Nullable<Array<Nullable<Text>>>)]
    pub institution_names: Vec<Option<String>>,
    #[diesel(sql_type = Numeric)]
    pub amount: BigDecimal,
    pub credit_type: String,
    #[diesel(sql_type = Timestamp)]
    pub application_datetime: NaiveDateTime,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct InsertConsumerCreditModel {
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub sin_ssn: Option<String>,
    pub institution_names: Vec<Option<String>>,
    #[diesel(sql_type = Numeric)]
    pub amount: BigDecimal,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
    pub user_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = consumer_credit)]
pub struct UpdateConsumerCreditModel {
    pub consumer_credit_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub address: Option<String>,
    pub phone_number: Option<String>,
    pub sin_ssn: Option<String>,
    pub institution_names: Option<Vec<Option<String>>>,
    #[diesel(sql_type = Numeric)]
    pub amount: Option<BigDecimal>,
    pub credit_type: Option<String>,
    pub application_datetime: Option<NaiveDateTime>,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: Option<String>,
}

#[derive(Deserialize, Serialize, Queryable, Insertable, Debug)]
#[diesel(table_name = consumer_credit_events)]
pub struct ConsumerCreditEventModel {
    pub id: i32,
    pub consumer_credit_id: String,
    pub event_type: String,
    pub event_data: Value,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Queryable, Insertable, Debug)]
#[diesel(table_name = consumer_credit_events)]
pub struct InsertConsumerCreditEventModel {
    pub consumer_credit_id: String,
    pub event_type: String,
    pub event_data: Value,
}
