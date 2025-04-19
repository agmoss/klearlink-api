use crate::consumer_credit::dto::ConsumerMatchStatisticsDto;
use crate::consumer_credit::models::ConsumerCreditModel;

use chrono::Local;

pub struct ConsumerMatchStatistics<'a> {
    records: &'a [ConsumerCreditModel],
    now: chrono::NaiveDateTime,
}

impl<'a> ConsumerMatchStatistics<'a> {
    pub fn new(records: &'a [ConsumerCreditModel]) -> Self {
        Self {
            records,
            now: Local::now().naive_local(),
        }
    }

    fn calculate_days_since_last_application(&self) -> i64 {
        let last_application = self
            .records
            .iter()
            .map(|r| r.application_datetime)
            .max()
            .unwrap();
        (self.now - last_application).num_days()
    }

    fn calculate_days_since_last_origination(&self) -> Option<i64> {
        let last_origination = self
            .records
            .iter()
            .filter_map(|r| r.originated_datetime)
            .max();
        last_origination.map(|dt| (self.now - dt).num_days())
    }

    fn calculate_active_loans(&self) -> Vec<&ConsumerCreditModel> {
        self.records
            .iter()
            .filter(|r| {
                r.credit_state == "originated"
                    || r.credit_state == "compliant"
                    || r.credit_state == "non-compliant"
            })
            .collect()
    }

    fn calculate_number_of_active_loans(&self) -> usize {
        self.calculate_active_loans().len()
    }

    fn calculate_average_credit_age(&self) -> Option<f64> {
        let active_loans = self.calculate_active_loans();
        if active_loans.is_empty() {
            return None;
        }

        let total_age: i64 = active_loans
            .iter()
            .filter_map(|r| r.originated_datetime)
            .map(|dt| (self.now - dt).num_days())
            .sum();

        Some(total_age as f64 / active_loans.len() as f64)
    }

    pub fn to_dto(&self) -> ConsumerMatchStatisticsDto {
        ConsumerMatchStatisticsDto {
            days_since_last_application: self.calculate_days_since_last_application(),
            days_since_last_origination: self.calculate_days_since_last_origination(),
            average_credit_age: self.calculate_average_credit_age(),
            number_of_active_loans: self.calculate_number_of_active_loans(),
        }
    }
}
