use crate::core::dto_validators::Validator;

use super::models::{ConsumerCreditModel, InsertConsumerCreditModel, UpdateConsumerCreditModel};
use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerFactsDto {
    #[validate(min_length = 2)]
    pub first_name: String,
    #[validate(min_length = 2)]
    pub last_name: String,
    #[validate(custom = Validator::email_validation)]
    pub email: String,
    #[validate(custom = Validator::past_or_present_date)]
    pub date_of_birth: NaiveDate,
    #[validate(custom = Validator::address_validation)]
    pub address: String,
    #[validate(custom = Validator::phone_validation)]
    pub phone_number: String,
    #[validate(custom = Validator::sin_validation)]
    pub sin_ssn: Option<String>,
    #[validate(unique_items)]
    pub institution_names: Vec<Option<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct CreditFactsDto {
    #[validate(custom = Validator::non_negative_bigdecimal)]
    pub amount: BigDecimal,
    #[validate(custom = Validator::credit_type_validation)]
    pub credit_type: String,
    #[validate(custom = Validator::past_or_present_datetime)]
    pub application_datetime: NaiveDateTime,
    #[validate(custom = Validator::credit_state_validation)]
    pub credit_state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerCreditDto {
    #[validate]
    pub consumer_facts: ConsumerFactsDto,
    #[validate]
    pub credit_facts: CreditFactsDto,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerMatchDto {
    pub consumer_facts: ConsumerFactsDto,
    pub credit_facts: CreditFactsDto,
    pub consumer_match: Option<Vec<ConsumerMatchesDto>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerMatchesDto {
    pub matched_on: MatchedOnDto,
    pub credit_facts: MatchedCreditFactsDto,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct MatchedCreditFactsDto {
    pub amount: BigDecimal,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    pub credit_state: String,
    pub institution_names: Vec<Option<String>>,
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

impl ConsumerCreditDto {
    pub fn to_insert_consumer_credit(
        &self,
        consumer_credit_id_dto: &str,
        user_id: &i32,
    ) -> InsertConsumerCreditModel {
        InsertConsumerCreditModel {
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
            user_id: *user_id,
        }
    }

    pub fn to_update_consumer_credit_model(
        &self,
        consumer_credit_id_dto: &str,
    ) -> UpdateConsumerCreditModel {
        UpdateConsumerCreditModel {
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

impl From<ConsumerCreditModel> for ConsumerCreditDto {
    fn from(consumer_credit: ConsumerCreditModel) -> Self {
        ConsumerCreditDto {
            consumer_facts: ConsumerFactsDto {
                first_name: consumer_credit.first_name,
                last_name: consumer_credit.last_name,
                email: consumer_credit.email,
                date_of_birth: consumer_credit.date_of_birth,
                address: consumer_credit.address,
                phone_number: consumer_credit.phone_number,
                sin_ssn: consumer_credit.sin_ssn,
                institution_names: consumer_credit.institution_names,
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
