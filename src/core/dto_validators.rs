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
}
