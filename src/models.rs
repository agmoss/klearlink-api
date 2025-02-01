use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::consumer_credit;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct ConsumerCredit {
    pub id: i32,
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone_number: String,
    pub consumer_state: String,
    pub institution_names: Vec<Option<String>>,
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: String,
    pub credit_state: String,
    pub tenant: String,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct InsertConsumerCredit {
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone_number: String,
    pub consumer_state: String,
    pub institution_names: Vec<String>,
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: String,
    pub credit_state: String,
    pub tenant: String,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
pub struct UpdateConsumerCredit {
    pub consumer_credit_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone_number: String,
    pub consumer_state: String,
    pub institution_names: Vec<String>,
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: String,
    pub credit_state: String,
}
