use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::schema::consumer_facts;
use super::schema::credit_facts;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = consumer_facts)]
pub struct ConsumerFacts {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone_number: String,
    pub consumer_state: String,
    pub institution_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = credit_facts)]
pub struct CreditFacts {
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: String,
    pub credit_state: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ConsumerCreditRecord {
    pub consumer_facts: ConsumerFacts,
    pub credit_facts: CreditFacts,
}
