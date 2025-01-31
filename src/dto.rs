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
pub struct ConsumerFactsDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone_number: String,
    pub consumer_state: String,
    pub institution_names: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditFactsDto {
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: String,
    pub credit_state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerCreditRecord {
    pub consumer_facts: ConsumerFactsDto,
    pub credit_facts: CreditFactsDto,
}

impl From<ConsumerCreditRecord> for ConsumerCreditDto {
    fn from(record: ConsumerCreditRecord) -> Self {
        ConsumerCreditDto {
            first_name: record.consumer_facts.first_name,
            last_name: record.consumer_facts.last_name,
            email: record.consumer_facts.email,
            date_of_birth: record.consumer_facts.date_of_birth,
            address: record.consumer_facts.address,
            phone_number: record.consumer_facts.phone_number,
            consumer_state: record.consumer_facts.consumer_state,
            institution_names: record.consumer_facts.institution_names,
            amount: record.credit_facts.amount,
            credit_type: record.credit_facts.credit_type,
            application_datetime: record.credit_facts.application_datetime,
            credit_state: record.credit_facts.credit_state,
        }
    }
}

impl ConsumerCreditRecord {
    pub fn to_consumer_credit(&self, consumer_credit_id_dto: &str) -> NewConsumerCredit {
        let dto: ConsumerCreditDto = self.clone().into();
        NewConsumerCredit {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: dto.first_name.clone(),
            last_name: dto.last_name.clone(),
            email: dto.email.clone(),
            date_of_birth: dto.date_of_birth.clone(),
            address: dto.address.clone(),
            phone_number: dto.phone_number.clone(),
            consumer_state: dto.consumer_state.clone(),
            institution_names: dto.institution_names.clone(),
            amount: dto.amount,
            credit_type: dto.credit_type.clone(),
            application_datetime: dto.application_datetime.clone(),
            credit_state: dto.credit_state.clone(),
        }
    }
}

impl From<ConsumerCredit> for ConsumerCreditRecord {
    fn from(consumer_credit: ConsumerCredit) -> Self {
        ConsumerCreditRecord {
            consumer_facts: ConsumerFactsDto {
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
            },
            credit_facts: CreditFactsDto {
                amount: consumer_credit.amount,
                credit_type: consumer_credit.credit_type,
                application_datetime: consumer_credit.application_datetime,
                credit_state: consumer_credit.credit_state,
            },
        }
    }
}
