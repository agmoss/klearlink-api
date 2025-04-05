use crate::core::dto_validators::Validator;

use super::models::{
    ConsumerCreditModel, InsertConsumerCreditEventModel, InsertConsumerCreditModel,
    UpdateConsumerCreditModel,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sin_ssn: Option<String>,
    #[validate(unique_items)]
    pub institution_names: Vec<Option<String>>,
    #[validate(max_length = 2)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_information_indicator: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdateConsumerFactsDto {
    #[validate(min_length = 2)]
    pub first_name: Option<String>,
    #[validate(min_length = 2)]
    pub last_name: Option<String>,
    #[validate(custom = Validator::optional_email_validation)]
    pub email: Option<String>,
    #[validate(custom = Validator::optional_past_or_present_date)]
    pub date_of_birth: Option<NaiveDate>,
    #[validate(custom = Validator::optional_address_validation)]
    pub address: Option<String>,
    #[validate(custom = Validator::optional_phone_validation)]
    pub phone_number: Option<String>,
    #[validate(custom = Validator::sin_validation)]
    pub sin_ssn: Option<String>,
    #[validate(unique_items)]
    pub institution_names: Option<Vec<Option<String>>>,
    #[validate(max_length = 2)]
    pub consumer_information_indicator: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[validate(custom = Validator::validate_credit_facts)]
#[validate(custom = Validator::validate_credit_state)]
pub struct CreditFactsDto {
    #[validate(custom = Validator::non_negative_float)]
    pub amount: f64,
    #[validate(custom = Validator::credit_type_validation)]
    pub credit_type: String,
    #[validate(custom = Validator::past_or_present_datetime)]
    pub application_datetime: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_datetime: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_amount: Option<f64>,
    #[validate(custom = Validator::credit_state_validation)]
    pub credit_state: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
pub struct UpdateCreditFactsDto {
    #[validate(custom = Validator::optional_non_negative_float)]
    pub amount: Option<f64>,
    #[validate(custom = Validator::optional_credit_type_validation)]
    pub credit_type: Option<String>,
    #[validate(custom = Validator::optional_past_or_present_datetime)]
    pub application_datetime: Option<NaiveDateTime>,
    #[validate(custom = Validator::optional_past_or_present_datetime)]
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    #[validate(custom = Validator::optional_credit_state_validation)]
    pub credit_state: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct InsertConsumerCreditDto {
    #[validate]
    pub consumer_facts: ConsumerFactsDto,
    #[validate]
    pub credit_facts: CreditFactsDto,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct UpdateConsumerCreditDto {
    #[validate]
    pub consumer_facts: Option<UpdateConsumerFactsDto>,
    #[validate]
    pub credit_facts: Option<UpdateCreditFactsDto>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConsumerCreditDto {
    pub consumer_facts: ConsumerFactsDto,
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
    pub amount: f64,
    pub credit_type: String,
    pub application_datetime: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_datetime: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
    pub institution_names: Vec<Option<String>>,
    pub consumer_information_indicator: Option<String>,
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

impl UpdateConsumerCreditDto {
    fn extract<T: Clone>(opt: &Option<T>) -> Option<T> {
        opt.clone()
    }

    pub fn to_update_consumer_credit_model(
        &self,
        consumer_credit_id: &str,
    ) -> UpdateConsumerCreditModel {
        let facts = self.consumer_facts.as_ref();
        let credit = self.credit_facts.as_ref();

        UpdateConsumerCreditModel {
            consumer_credit_id: Some(consumer_credit_id.to_string()),
            first_name: Self::extract(&facts.and_then(|f| f.first_name.clone())),
            last_name: Self::extract(&facts.and_then(|f| f.last_name.clone())),
            email: Self::extract(&facts.and_then(|f| f.email.clone())),
            date_of_birth: facts.and_then(|f| f.date_of_birth),
            address: Self::extract(&facts.and_then(|f| f.address.clone())),
            phone_number: Self::extract(&facts.and_then(|f| f.phone_number.clone())),
            sin_ssn: Self::extract(&facts.and_then(|f| f.sin_ssn.clone())),
            institution_names: Self::extract(&facts.and_then(|f| f.institution_names.clone())),
            consumer_information_indicator: Self::extract(
                &facts.and_then(|f| f.consumer_information_indicator.clone()),
            ),
            amount: Self::extract(&credit.and_then(|c| c.amount)),
            credit_type: Self::extract(&credit.and_then(|c| c.credit_type.clone())),
            application_datetime: credit.and_then(|c| c.application_datetime),
            originated_datetime: credit.and_then(|c| c.originated_datetime),
            payment_due_date: credit.and_then(|c| c.payment_due_date),
            payment_due_amount: credit.and_then(|c| c.payment_due_amount),
            credit_state: Self::extract(&credit.and_then(|c| c.credit_state.clone())),
        }
    }
}

impl InsertConsumerCreditDto {
    pub fn to_insert_consumer_credit_model(
        &self,
        consumer_credit_id: &str,
        user_id: &i32,
    ) -> InsertConsumerCreditModel {
        InsertConsumerCreditModel {
            consumer_credit_id: consumer_credit_id.to_string(),
            first_name: self.consumer_facts.first_name.clone(),
            last_name: self.consumer_facts.last_name.clone(),
            email: self.consumer_facts.email.clone(),
            date_of_birth: self.consumer_facts.date_of_birth,
            address: self.consumer_facts.address.clone(),
            phone_number: self.consumer_facts.phone_number.clone(),
            sin_ssn: self.consumer_facts.sin_ssn.clone(),
            institution_names: self.consumer_facts.institution_names.clone(),
            consumer_information_indicator: self
                .consumer_facts
                .consumer_information_indicator
                .clone(),
            amount: self.credit_facts.amount,
            credit_type: self.credit_facts.credit_type.clone(),
            application_datetime: self.credit_facts.application_datetime,
            originated_datetime: self.credit_facts.originated_datetime,
            payment_due_date: self.credit_facts.payment_due_date,
            payment_due_amount: self.credit_facts.payment_due_amount,
            credit_state: self.credit_facts.credit_state.clone(),
            user_id: *user_id,
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
                consumer_information_indicator: consumer_credit.consumer_information_indicator,
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

impl ConsumerCreditModel {
    pub fn to_consumer_match_dto(&self, matches: Vec<ConsumerMatchesDto>) -> ConsumerMatchDto {
        ConsumerMatchDto {
            consumer_facts: ConsumerFactsDto {
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
                email: self.email.clone(),
                date_of_birth: self.date_of_birth,
                address: self.address.clone(),
                phone_number: self.phone_number.clone(),
                sin_ssn: self.sin_ssn.clone(),
                institution_names: self.institution_names.clone(),
                consumer_information_indicator: self.consumer_information_indicator.clone(),
            },
            credit_facts: CreditFactsDto {
                amount: self.amount.clone(),
                credit_type: self.credit_type.clone(),
                application_datetime: self.application_datetime,
                originated_datetime: self.originated_datetime,
                payment_due_date: self.payment_due_date,
                payment_due_amount: self.payment_due_amount,
                credit_state: self.credit_state.clone(),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
            processed: true,
            consumer_match: Some(matches),
        }
    }

    pub fn to_consumer_matches_dto(&self, _target: &ConsumerCreditModel) -> ConsumerMatchesDto {
        ConsumerMatchesDto {
            matched_on: MatchedOnDto {
                first_name: self.first_name == _target.first_name,
                last_name: self.last_name == _target.last_name,
                email: self.email == _target.email,
                date_of_birth: self.date_of_birth == _target.date_of_birth,
                address: self.address == _target.address,
                phone_number: self.phone_number == _target.phone_number,
            },
            credit_facts: MatchedCreditFactsDto {
                amount: self.amount.clone(),
                credit_type: self.credit_type.clone(),
                application_datetime: self.application_datetime,
                originated_datetime: self.originated_datetime,
                payment_due_date: self.payment_due_date,
                payment_due_amount: self.payment_due_amount,
                credit_state: self.credit_state.clone(),
                institution_names: self.institution_names.clone(),
                consumer_information_indicator: self.consumer_information_indicator.clone(),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct ConsumerCreditEventsDto {
    pub consumer_credit_id: String,
    pub event_type: String,
    pub event_data: Value,
}

impl ConsumerCreditEventsDto {
    pub fn to_insert_consumer_credit_events_model(&self) -> InsertConsumerCreditEventModel {
        InsertConsumerCreditEventModel {
            consumer_credit_id: self.consumer_credit_id.to_string(),
            event_type: self.event_type.to_string(),
            event_data: self.event_data.clone(),
        }
    }
}
