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
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<NaiveDateTime>,
    #[validate(custom = Validator::credit_state_validation)]
    pub credit_state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerCreditDto {
    #[validate]
    pub consumer_facts: ConsumerFactsDto,
    #[validate]
    pub credit_facts: CreditFactsDto,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub processed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerMatchDto {
    pub consumer_facts: ConsumerFactsDto,
    pub credit_facts: CreditFactsDto,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub processed: bool,
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
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<NaiveDateTime>,
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
            originated_datetime: self.credit_facts.originated_datetime.clone(),
            payment_due_date: self.credit_facts.payment_due_date.clone(),
            payment_due_amount: self.credit_facts.payment_due_amount.clone(),
            credit_state: self.credit_facts.credit_state.clone(),
            user_id: *user_id,
        }
    }

    pub fn from_model_and_matches(
        model: ConsumerCreditModel,
        matches: Vec<ConsumerMatchesDto>,
    ) -> ConsumerMatchDto {
        ConsumerMatchDto {
            consumer_facts: ConsumerFactsDto {
                first_name: model.first_name.clone(),
                last_name: model.last_name.clone(),
                email: model.email.clone(),
                date_of_birth: model.date_of_birth,
                address: model.address.clone(),
                phone_number: model.phone_number.clone(),
                sin_ssn: model.sin_ssn.clone(),
                institution_names: model.institution_names.clone(),
            },
            credit_facts: CreditFactsDto {
                amount: model.amount.clone(),
                credit_type: model.credit_type.clone(),
                application_datetime: model.application_datetime,
                originated_datetime: model.originated_datetime.clone(),
                payment_due_date: model.payment_due_date.clone(),
                payment_due_amount: model.payment_due_amount.clone(),
                credit_state: model.credit_state.clone(),
            },
            created_at: model.created_at,
            updated_at: model.updated_at,
            processed: true,
            consumer_match: Some(matches),
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
            application_datetime: self.credit_facts.application_datetime.clone(),
            originated_datetime: self.credit_facts.originated_datetime.clone(),
            payment_due_date: self.credit_facts.payment_due_date.clone(),
            payment_due_amount: self.credit_facts.payment_due_amount.clone(),
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
                originated_datetime: consumer_credit.originated_datetime,
                payment_due_date: consumer_credit.payment_due_date,
                payment_due_amount: consumer_credit.payment_due_amount,
                credit_state: consumer_credit.credit_state,
            },
            processed: true,
            created_at: consumer_credit.created_at,
            updated_at: consumer_credit.updated_at,
        }
    }
}
