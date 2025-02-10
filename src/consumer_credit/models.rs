use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::consumer_credit;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct ConsumerCredit {
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
    pub credit_state: String,
    pub tenant: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct InsertConsumerCredit {
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub sin_ssn: Option<String>,
    pub institution_names: Vec<String>,
    #[diesel(sql_type = Numeric)]
    pub amount: BigDecimal,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub credit_state: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct UpdateConsumerCredit {
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub sin_ssn: Option<String>,
    pub institution_names: Vec<String>,
    #[diesel(sql_type = Numeric)]
    pub amount: BigDecimal,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub credit_state: String,
}
