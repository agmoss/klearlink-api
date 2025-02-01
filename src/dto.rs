use serde::{Deserialize, Serialize};

use crate::models::{ConsumerCredit, NewConsumerCredit, NewConsumerCredit2};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumerCreditDto {
    #[serde(flatten)]
    pub consumer_facts: ConsumerFactsDto,
    #[serde(flatten)]
    pub credit_facts: CreditFactsDto,
}

impl From<ConsumerCreditRecord> for ConsumerCreditDto {
    fn from(record: ConsumerCreditRecord) -> Self {
        ConsumerCreditDto {
            consumer_facts: record.consumer_facts,
            credit_facts: record.credit_facts,
        }
    }
}

impl ConsumerCreditRecord {
    pub fn to_new_consumer_credit(
        &self,
        consumer_credit_id_dto: &str,
        tenant: &str,
    ) -> NewConsumerCredit {
        NewConsumerCredit {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: self.consumer_facts.first_name.clone(),
            last_name: self.consumer_facts.last_name.clone(),
            email: self.consumer_facts.email.clone(),
            date_of_birth: self.consumer_facts.date_of_birth.clone(),
            address: self.consumer_facts.address.clone(),
            phone_number: self.consumer_facts.phone_number.clone(),
            consumer_state: self.consumer_facts.consumer_state.clone(),
            institution_names: self.consumer_facts.institution_names.clone(),
            amount: self.credit_facts.amount,
            credit_type: self.credit_facts.credit_type.clone(),
            application_datetime: self.credit_facts.application_datetime.clone(),
            credit_state: self.credit_facts.credit_state.clone(),
            tenant: tenant.to_string(),
        }
    }

    pub fn to_new_consumer_credit2(&self, consumer_credit_id_dto: &str) -> NewConsumerCredit2 {
        NewConsumerCredit2 {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: self.consumer_facts.first_name.clone(),
            last_name: self.consumer_facts.last_name.clone(),
            email: self.consumer_facts.email.clone(),
            date_of_birth: self.consumer_facts.date_of_birth.clone(),
            address: self.consumer_facts.address.clone(),
            phone_number: self.consumer_facts.phone_number.clone(),
            consumer_state: self.consumer_facts.consumer_state.clone(),
            institution_names: self.consumer_facts.institution_names.clone(),
            amount: self.credit_facts.amount,
            credit_type: self.credit_facts.credit_type.clone(),
            application_datetime: self.credit_facts.application_datetime.clone(),
            credit_state: self.credit_facts.credit_state.clone(),
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
                    .flatten()
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
