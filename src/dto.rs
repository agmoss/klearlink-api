use serde::{Deserialize, Serialize};

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
