use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::models::{ConsumerCredit, InsertConsumerCredit, UpdateConsumerCredit};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerFactsDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: NaiveDate,
    pub address: String,
    pub phone_number: String,
    pub sin_ssn: Option<String>,
    pub institution_names: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreditFactsDto {
    pub amount: BigDecimal,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub credit_state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerCreditDto {
    pub consumer_facts: ConsumerFactsDto,
    pub credit_facts: CreditFactsDto,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerMatchDto {
    pub consumer_credit: ConsumerCreditDto,
    pub matched_on: MatchedOnDto,
}

#[derive(Debug, Deserialize, Serialize, Clone)]                                                                                                       
pub struct MatchedOnDto {                                                                                                                             
    pub first_name: bool,                                                                                                                             
    pub last_name: bool,                                                                                                                              
    pub email: bool,                                                                                                                                  
    pub date_of_birth: bool,                                                                                                                          
    pub address: bool,                                                                                                                                
    pub phone_number: bool,                                                                                                                                                                                                                                          
}                                                                                                                                                     
                                                                                                                                               
    

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct MatchedOnDto {
//     pub first_name: bool,
//     pub last_name: bool,
//     pub email: bool,
//     pub date_of_birth: bool,
//     pub address: bool,
//     pub phone_number: bool,
//     pub institution_names: Vec<String>,
//     pub consumer_facts: ConsumerFactsDto,
//     pub credit_facts: CreditFactsDto,
// }

impl ConsumerCreditDto {
    pub fn to_insert_consumer_credit(
        &self,
        consumer_credit_id_dto: &str,
        tenant: &str,
    ) -> InsertConsumerCredit {
        InsertConsumerCredit {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: self.consumer_facts.first_name.clone(),
            last_name: self.consumer_facts.last_name.clone(),
            email: self.consumer_facts.email.clone(),
            date_of_birth: self.consumer_facts.date_of_birth,
            address: self.consumer_facts.address.clone(),
            phone_number: self.consumer_facts.phone_number.clone(),
            sin_ssn: self.consumer_facts.sin_ssn.clone(),
            institution_names: self.consumer_facts.institution_names.clone(),
            amount: self.credit_facts.amount.clone(),
            credit_type: self.credit_facts.credit_type.clone(),
            application_datetime: self.credit_facts.application_datetime,
            credit_state: self.credit_facts.credit_state.clone(),
            tenant: tenant.to_string(),
        }
    }

    pub fn to_update_consumer_credit_model(
        &self,
        consumer_credit_id_dto: &str,
    ) -> UpdateConsumerCredit {
        UpdateConsumerCredit {
            consumer_credit_id: consumer_credit_id_dto.to_string(),
            first_name: self.consumer_facts.first_name.clone(),
            last_name: self.consumer_facts.last_name.clone(),
            email: self.consumer_facts.email.clone(),
            date_of_birth: self.consumer_facts.date_of_birth,
            address: self.consumer_facts.address.clone(),
            phone_number: self.consumer_facts.phone_number.clone(),
            sin_ssn: self.consumer_facts.sin_ssn.clone(),
            institution_names: self.consumer_facts.institution_names.clone(),
            amount: self.credit_facts.amount.clone(),
            credit_type: self.credit_facts.credit_type.clone(),
            application_datetime: self.credit_facts.application_datetime,
            credit_state: self.credit_facts.credit_state.clone(),
        }
    }
}

impl From<ConsumerCredit> for ConsumerCreditDto {
    fn from(consumer_credit: ConsumerCredit) -> Self {
        ConsumerCreditDto {
            consumer_facts: ConsumerFactsDto {
                first_name: consumer_credit.first_name,
                last_name: consumer_credit.last_name,
                email: consumer_credit.email,
                date_of_birth: consumer_credit.date_of_birth,
                address: consumer_credit.address,
                phone_number: consumer_credit.phone_number,
                sin_ssn: consumer_credit.sin_ssn,
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
