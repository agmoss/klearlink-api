use bigdecimal::BigDecimal;
use chrono::Local;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use regex::Regex;
use serde_valid::validation::Error;

pub struct Validator;

impl Validator {
    pub fn email_validation(val: &str) -> Result<(), Error> {
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

    pub fn phone_validation(val: &str) -> Result<(), Error> {
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

    pub fn address_validation(val: &str) -> Result<(), Error> {
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

    pub fn credit_type_validation(val: &str) -> Result<(), Error> {
        match val {
            "PDL" | "BNPL" => Ok(()),
            _ => Err(Error::Custom(format!(
                "Invalid credit_type: {}. Must be 'PDL' or 'BNPL'.",
                val
            ))),
        }
    }

    pub fn credit_state_validation(val: &str) -> Result<(), Error> {
        match val {
            "application" | "originated" | "declined" | "non-compliant" | "compliant" | "bankrupt/insolvent" => Ok(()),
            _ => Err(Error::Custom(format!(
                "Invalid status: {}. Must be one of ['application', 'originated', 'declined', 'non-compliant', 'compliant', 'bankrupt/insolvent'].",
                val
            ))),
        }
    }

    pub fn non_negative_bigdecimal(val: &BigDecimal) -> Result<(), Error> {
        if val >= &BigDecimal::from(0) {
            Ok(())
        } else {
            Err(Error::Custom(format!(
                "Invalid value: {}. Must be a non-negative number.",
                val
            )))
        }
    }

    pub fn past_or_present_date(val: &NaiveDate) -> Result<(), Error> {
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

    pub fn past_or_present_datetime(val: &NaiveDateTime) -> Result<(), Error> {
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

    pub fn sin_validation(val: &Option<String>) -> Result<(), Error> {
        if let Some(sin) = val {
            // Ensure SIN contains exactly 9 digits
            if sin.len() != 9 || !sin.chars().all(|c| c.is_ascii_digit()) {
                return Err(Error::Custom(format!(
                    "Invalid SIN: {}. Must be exactly 9 digits.",
                    sin
                )));
            }

            // First digit must be 1-9 (no leading zero)
            if sin.starts_with('0') {
                return Err(Error::Custom(format!(
                    "Invalid SIN: {}. Cannot start with 0.",
                    sin
                )));
            }

            // Validate using Luhn Algorithm
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
