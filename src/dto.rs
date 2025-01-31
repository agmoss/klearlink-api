use serde::{Deserialize, Serialize};

use crate::models::{ConsumerCredit, NewConsumerCredit};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerCreditDto {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerCreditRecord {
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

impl ConsumerCreditRecord {
    pub fn to_consumer_credit(&self, consumer_credit_id_dto: &str) -> NewConsumerCredit {
        NewConsumerCredit {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            date_of_birth: self.date_of_birth.clone(),
            address: self.address.clone(),
            phone_number: self.phone_number.clone(),
            consumer_state: self.consumer_state.clone(),
            institution_names: self.institution_names.clone(),
            amount: self.amount,
            credit_type: self.credit_type.clone(),
            application_datetime: self.application_datetime.clone(),
            credit_state: self.credit_state.clone(),
        }
    }
}

impl From<ConsumerCredit> for ConsumerCreditRecord {
    fn from(consumer_credit: ConsumerCredit) -> Self {
        ConsumerCreditRecord {
            first_name: consumer_credit.first_name,
            last_name: consumer_credit.last_name,
            email: consumer_credit.email,
            date_of_birth: consumer_credit.date_of_birth,
            address: consumer_credit.address,
            phone_number: consumer_credit.phone_number,
            consumer_state: consumer_credit.consumer_state,
            institution_names: consumer_credit
                .institution_names
                .into_iter()
                .flatten() // Remove None values
                .collect(),
            amount: consumer_credit.amount,
            credit_type: consumer_credit.credit_type,
            application_datetime: consumer_credit.application_datetime,
            credit_state: consumer_credit.credit_state,
        }
    }
}
