use crate::schema::consumer_credit;
use crate::schema::consumer_credit_events;
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
    pub institution_name: Option<String>,
    pub amount: f64,
    pub credit_type: String,
    #[diesel(sql_type = Timestamp)]
    pub application_datetime: NaiveDateTime,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
    pub consumer_information_indicator: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub total_installments: Option<i32>,
    pub paid_installments: Option<i32>,
    pub installment_amount: Option<f64>,
    pub ip_address: Option<String>,
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
    pub institution_name: Option<String>,
    pub consumer_information_indicator: Option<String>,
    pub ip_address: Option<String>,
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
    pub user_id: i32,
    pub total_installments: Option<i32>,
    pub paid_installments: Option<i32>,
    pub installment_amount: Option<f64>,
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
    pub institution_name: Option<String>,
    pub amount: Option<f64>,
    pub credit_type: Option<String>,
    pub application_datetime: Option<NaiveDateTime>,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: Option<String>,
    pub consumer_information_indicator: Option<String>,
    pub ip_address: Option<String>,
    pub total_installments: Option<i32>,
    pub paid_installments: Option<i32>,
    pub installment_amount: Option<f64>,
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
