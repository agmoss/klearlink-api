use bigdecimal::BigDecimal;
use chrono::{Local, NaiveDate, NaiveDateTime};
use regex::Regex;
use serde_valid::validation::Error;

use crate::consumer_credit::dto::CreditFactsDto;

use super::res::ValidatorError;

pub struct Validator;

impl Validator {
    pub fn email_validation(val: &str) -> ValidatorError {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        if email_regex.is_match(val) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid RFC 5322/822 email: {}",
                val
            )))
        }
    }

    pub fn validate_credit_facts(dto: &CreditFactsDto) -> ValidatorError {
        let count = dto.originated_datetime.is_some() as u8
            + dto.payment_due_date.is_some() as u8
            + dto.payment_due_amount.is_some() as u8;

        if count == 0 || count == 3 {
            Ok(())
        } else {
            Err(Error::Custom(
                "All or none of originated_datetime, payment_due_date, and payment_due_amount must be supplied.".to_string(),
            ))
        }
    }

    pub fn validate_credit_state(dto: &CreditFactsDto) -> ValidatorError {
        if dto.credit_state == "applied" {
            let count = dto.originated_datetime.is_some() as u8
                + dto.payment_due_date.is_some() as u8
                + dto.payment_due_amount.is_some() as u8;

            if count == 0 {
                Ok(())
            } else {
                Err(Error::Custom(
                    "If credit_state is 'applied', originated_datetime, payment_due_date, and payment_due_amount cannot be present.".to_string(),
                ))
            }
        } else {
            Ok(())
        }
    }

    pub fn optional_email_validation(val: &Option<String>) -> ValidatorError {
        if let Some(value) = val {
            Self::email_validation(value)
        } else {
            Ok(())
        }
    }

    pub fn phone_validation(val: &str) -> ValidatorError {
        let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
        if phone_regex.is_match(val) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid E.164 phone number: {}",
                val
            )))
        }
    }

    pub fn optional_phone_validation(val: &Option<String>) -> ValidatorError {
        if let Some(value) = val {
            Self::phone_validation(value)
        } else {
            Ok(())
        }
    }

    pub fn address_validation(val: &str) -> ValidatorError {
        let address_regex = Regex::new(r"^\d+\s[A-Za-z0-9\s.,'-]+,\s[A-Za-z\s-]+,\s(?:AB|BC|MB|NB|NL|NS|NT|NU|ON|PE|QC|SK|YT),\s[A-Za-z]\d[A-Za-z]\s?\d[A-Za-z]\d(?:,\sCanada)?$").unwrap();
        if address_regex.is_match(val) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid CAN/CSA-Z109.1-01 address: {}",
                val
            )))
        }
    }

    pub fn optional_address_validation(val: &Option<String>) -> ValidatorError {
        if let Some(value) = val {
            Self::address_validation(value)
        } else {
            Ok(())
        }
    }

    pub fn credit_type_validation(val: &str) -> ValidatorError {
        match val {
            "PDL" | "BNPL" => Ok(()),
            _ => Err(Error::Custom(format!(
                "Invalid credit_type: {}. Must be 'PDL' or 'BNPL'.",
                val
            ))),
        }
    }

    pub fn optional_credit_type_validation(val: &Option<String>) -> ValidatorError {
        if let Some(value) = val {
            Self::credit_type_validation(value)
        } else {
            Ok(())
        }
    }

    pub fn credit_state_validation(val: &str) -> ValidatorError {
        match val {
            "application" | "originated" | "declined" | "non-compliant" | "compliant" | "bankrupt/insolvent" => Ok(()),
            _ => Err(Error::Custom(format!(
                "Invalid status: {}. Must be one of ['application', 'originated', 'declined', 'non-compliant', 'compliant', 'bankrupt/insolvent'].",
                val
            ))),
        }
    }

    pub fn optional_credit_state_validation(val: &Option<String>) -> ValidatorError {
        if let Some(value) = val {
            Self::credit_state_validation(value)
        } else {
            Ok(())
        }
    }

    pub fn non_negative_bigdecimal(val: &BigDecimal) -> ValidatorError {
        if val >= &BigDecimal::from(0) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid value: {}. Must be a non-negative number.",
                val
            )))
        }
    }

    pub fn optional_non_negative_bigdecimal(val: &Option<BigDecimal>) -> ValidatorError {
        if let Some(value) = val {
            Self::non_negative_bigdecimal(value)
        } else {
            Ok(())
        }
    }

    pub fn past_or_present_date(val: &NaiveDate) -> ValidatorError {
        let today = Local::now().date_naive();
        if val <= &today {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid date: {}. The date cannot be in the future.",
                val
            )))
        }
    }

    pub fn optional_past_or_present_date(val: &Option<NaiveDate>) -> ValidatorError {
        if let Some(value) = val {
            Self::past_or_present_date(value)
        } else {
            Ok(())
        }
    }

    pub fn past_or_present_datetime(val: &NaiveDateTime) -> ValidatorError {
        let now = Local::now().naive_local();
        if val <= &now {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid datetime: {}. The datetime cannot be in the future.",
                val
            )))
        }
    }

    pub fn optional_past_or_present_datetime(val: &Option<NaiveDateTime>) -> ValidatorError {
        if let Some(value) = val {
            Self::past_or_present_datetime(value)
        } else {
            Ok(())
        }
    }

    pub fn sin_validation(val: &Option<String>) -> ValidatorError {
        if let Some(sin) = val {
            if sin.len() != 9 || !sin.chars().all(|c| c.is_ascii_digit()) {
                return Err(Error::Custom(format!(
                    "Invalid SIN: {}. Must be exactly 9 digits.",
                    sin
                )));
            }
            if sin.starts_with('0') {
                return Err(Error::Custom(format!(
                    "Invalid SIN: {}. Cannot start with 0.",
                    sin
                )));
            }
            if !Self::luhn_check(sin) {
                return Err(Error::Custom(format!(
                    "Invalid SIN: {}. Failed Luhn checksum validation.",
                    sin
                )));
            }
        }
        Ok(())
    }

    fn luhn_check(sin: &str) -> bool {
        let digits: Vec<u32> = sin.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 9 {
            return false;
        }

        let sum: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 1 {
                    let double = digit * 2;
                    if double > 9 {
                        double - 9
                    } else {
                        double
                    }
                } else {
                    digit
                }
            })
            .sum();

        sum % 10 == 0
    }
}
