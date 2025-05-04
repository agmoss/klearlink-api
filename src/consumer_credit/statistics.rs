use crate::consumer_credit::dto::ConsumerMatchStatisticsDto;
use crate::consumer_credit::models::ConsumerCreditModel;

use chrono::{Duration, Local};

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

        let average = total_age as f64 / active_loans.len() as f64;
        Some((average * 100.0).round() / 100.0)
    }

    fn calculate_application_frequency_last_12_months(&self) -> usize {
        let one_year_ago = self.now - Duration::days(365);
        self.records
            .iter()
            .filter(|r| r.application_datetime >= one_year_ago)
            .count()
    }

    fn calculate_origination_frequency_last_12_months(&self) -> usize {
        let one_year_ago = self.now - Duration::days(365);
        self.records
            .iter()
            .filter(|r| {
                r.credit_state == "originated"
                    && r.originated_datetime.is_some()
                    && r.originated_datetime.unwrap() >= one_year_ago
            })
            .count()
    }

    fn calculate_credit_stacking_indicator(&self) -> usize {
        let thirty_days_ago = self.now - Duration::days(30);
        self.records
            .iter()
            .filter(|r| {
                r.credit_state == "originated"
                    && r.originated_datetime.is_some()
                    && r.originated_datetime.unwrap() >= thirty_days_ago
            })
            .count()
    }

    fn calculate_missed_payment_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.credit_state == "non-compliant")
            .count()
    }

    fn calculate_days_in_non_compliance(&self) -> i64 {
        self.records
            .iter()
            .filter(|r| r.credit_state == "non-compliant")
            .filter_map(|r| r.payment_due_date)
            .map(|due_date| {
                if due_date < self.now {
                    (self.now - due_date).num_days()
                } else {
                    0
                }
            })
            .sum()
    }

    fn calculate_percentage_of_non_compliant_payments(&self) -> f64 {
        let total_payments = self
            .records
            .iter()
            .filter(|r| r.payment_due_date.is_some())
            .count();

        if total_payments == 0 {
            return 0.0;
        }

        let non_compliant_payments = self
            .records
            .iter()
            .filter(|r| r.credit_state == "non-compliant" && r.payment_due_date.is_some())
            .count();

        let percentage = (non_compliant_payments as f64 / total_payments as f64) * 100.0;
        (percentage * 100.0).round() / 100.0
    }

    fn calculate_current_delinquency_status(&self) -> bool {
        self.records
            .iter()
            .any(|r| r.credit_state == "non-compliant")
    }

    fn calculate_historical_delinquency_rate(&self) -> f64 {
        let total_periods = self.records.len();
        if total_periods == 0 {
            return 0.0;
        }

        let non_compliant_periods = self
            .records
            .iter()
            .filter(|r| r.credit_state == "non-compliant")
            .count();

        let rate = non_compliant_periods as f64 / total_periods as f64;
        (rate * 100.0).round() / 100.0
    }

    fn calculate_multi_account_phone_usage(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }

        let first_phone = &self.records[0].phone_number;
        self.records
            .iter()
            .filter(|r| r.phone_number != *first_phone)
            .count()
    }

    fn calculate_multi_account_email_usage(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }

        let first_email = &self.records[0].email;
        self.records
            .iter()
            .filter(|r| r.email != *first_email)
            .count()
    }

    fn is_insolvency_indicator(indicator: &Option<String>) -> bool {
        if let Some(ind) = indicator {
            matches!(
                ind.as_str(),
                "A" | "B" | "C" | "D" | "E" | "F" | "G" | "T" | "Z" | "ZA" | "ZB" | "ZC" | "ZD"
            )
        } else {
            false
        }
    }

    fn calculate_insolvency_status_indicator(&self) -> bool {
        self.records
            .iter()
            .any(|r| Self::is_insolvency_indicator(&r.consumer_information_indicator))
    }

    fn calculate_repeated_insolvency_flag(&self) -> bool {
        let insolvency_count = self
            .records
            .iter()
            .filter(|r| Self::is_insolvency_indicator(&r.consumer_information_indicator))
            .count();
        insolvency_count > 1
    }

    fn calculate_high_frequency_applicant(&self) -> bool {
        if self.records.len() < 2 {
            return false;
        }

        let mut application_times: Vec<_> = self
            .records
            .iter()
            .map(|r| r.application_datetime)
            .collect();

        application_times.sort();

        for window in application_times.windows(2) {
            if (window[1] - window[0]).num_hours() <= 24 {
                return true;
            }
        }

        false
    }

    pub fn to_dto(&self) -> ConsumerMatchStatisticsDto {
        ConsumerMatchStatisticsDto {
            days_since_last_application: self.calculate_days_since_last_application(),
            days_since_last_origination: self.calculate_days_since_last_origination(),
            average_credit_age: self.calculate_average_credit_age(),
            number_of_active_loans: self.calculate_number_of_active_loans(),
            application_frequency_last_12_months: self
                .calculate_application_frequency_last_12_months(),
            origination_frequency_last_12_months: self
                .calculate_origination_frequency_last_12_months(),
            credit_stacking_indicator: self.calculate_credit_stacking_indicator(),
            missed_payment_count: self.calculate_missed_payment_count(),
            days_in_non_compliance: self.calculate_days_in_non_compliance(),
            percentage_of_non_compliant_payments: self
                .calculate_percentage_of_non_compliant_payments(),
            current_delinquency_status: self.calculate_current_delinquency_status(),
            historical_delinquency_rate: self.calculate_historical_delinquency_rate(),
            multi_account_phone_usage: self.calculate_multi_account_phone_usage(),
            multi_account_email_usage: self.calculate_multi_account_email_usage(),
            insolvency_status_indicator: self.calculate_insolvency_status_indicator(),
            repeated_insolvency_flag: self.calculate_repeated_insolvency_flag(),
            high_frequency_applicant: self.calculate_high_frequency_applicant(),
        }
    }
}
