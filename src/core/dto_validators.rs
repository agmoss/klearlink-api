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

        if count != 0 && count != 3 {
            return Err(Error::Custom(
                "All or none of originated_datetime, payment_due_date, and payment_due_amount must be supplied.".to_string(),
            ));
        }

        // Validate BNPL specific fields
        if dto.credit_type == "BNPL" {
            let bnpl_count = dto.total_installments.is_some() as u8
                + dto.paid_installments.is_some() as u8
                + dto.installment_amount.is_some() as u8;

            if bnpl_count > 0 && bnpl_count < 3 {
                return Err(Error::Custom(
                    "For BNPL loans, all or none of total_installments, paid_installments, and installment_amount must be supplied.".to_string(),
                ));
            }

            if let (Some(total), Some(paid), Some(amount)) = (
                dto.total_installments,
                dto.paid_installments,
                dto.installment_amount,
            ) {
                if total <= 0 {
                    return Err(Error::Custom(
                        "total_installments must be greater than 0".to_string(),
                    ));
                }
                if paid < 0 || paid > total {
                    return Err(Error::Custom(
                        "paid_installments must be between 0 and total_installments".to_string(),
                    ));
                }
                if amount <= 0.0 {
                    return Err(Error::Custom(
                        "installment_amount must be greater than 0".to_string(),
                    ));
                }
                if (amount * total as f64).round() != dto.amount.round() {
                    return Err(Error::Custom(
                        "installment_amount * total_installments must equal the total amount"
                            .to_string(),
                    ));
                }
            }
        } else if dto.total_installments.is_some()
            || dto.paid_installments.is_some()
            || dto.installment_amount.is_some()
        {
            return Err(Error::Custom(
                "Installment fields can only be specified for BNPL loans.".to_string(),
            ));
        }

        Ok(())
    }

    pub fn validate_credit_state(dto: &CreditFactsDto) -> ValidatorError {
        if dto.credit_state == "application" {
            // For any credit type in application state, these fields should be null
            let count = dto.originated_datetime.is_some() as u8
                + dto.payment_due_date.is_some() as u8
                + dto.payment_due_amount.is_some() as u8;

            if count > 0 {
                return Err(Error::Custom(
                    "If credit_state is 'application', originated_datetime, payment_due_date, and payment_due_amount cannot be present.".to_string(),
                ));
            }

            // For BNPL loans in application state, these fields should also be null
            if dto.credit_type == "BNPL" {
                let bnpl_count = dto.total_installments.is_some() as u8
                    + dto.paid_installments.is_some() as u8
                    + dto.installment_amount.is_some() as u8;

                if bnpl_count > 0 {
                    return Err(Error::Custom(
                        "For BNPL loans in 'application' state, total_installments, paid_installments, and installment_amount cannot be present.".to_string(),
                    ));
                }
            }
        }
        Ok(())
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

    pub fn _non_negative_bigdecimal(val: &BigDecimal) -> ValidatorError {
        if val >= &BigDecimal::from(0) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid value: {}. Must be a non-negative number.",
                val
            )))
        }
    }

    pub fn _optional_non_negative_bigdecimal(val: &Option<BigDecimal>) -> ValidatorError {
        if let Some(value) = val {
            Self::_non_negative_bigdecimal(value)
        } else {
            Ok(())
        }
    }

    pub fn non_negative_float(val: &f64) -> ValidatorError {
        if val >= &0.0 {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid value: {}. Must be a non-negative number.",
                val
            )))
        }
    }

    pub fn optional_non_negative_float(val: &Option<f64>) -> ValidatorError {
        if let Some(value) = val {
            Self::non_negative_float(value)
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

    pub fn ip_validation(val: &Option<String>) -> ValidatorError {
        if let Some(ip) = val {
            // Check for IPv4 format (e.g., 192.168.1.1)
            let ipv4_regex = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").unwrap();
            if ipv4_regex.is_match(ip) {
                // Validate each octet is between 0-255
                let octets: Vec<u8> = ip.split('.').filter_map(|o| o.parse().ok()).collect();
                if octets.len() == 4 && octets.iter().all(|&o| o <= 255) {
                    return Ok(());
                }
            }

            // Check for IPv6 format (e.g., 2001:0db8:85a3:0000:0000:8a2e:0370:7334)
            let ipv6_regex = Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();
            if ipv6_regex.is_match(ip) {
                return Ok(());
            }

            return Err(Error::Custom(format!(
                "Invalid IP address: {}. Must be a valid IPv4 or IPv6 address.",
                ip
            )));
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
