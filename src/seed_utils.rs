use diesel::prelude::*;

use dotenv::dotenv;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use fake::faker::address::raw::*;
use fake::faker::number::en::NumberWithFormat;
use fake::locales::*;
use fake::Fake;

use chrono::{Duration, NaiveDate, NaiveDateTime};
use uuid::Uuid;

use indicatif::{ProgressBar, ProgressStyle};

pub fn generate_reproducible_uuid(seed: u128) -> Uuid {
    // Use a fixed seed to generate a reproducible UUID
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let bytes: [u8; 16] = rng.random();
    Uuid::from_bytes(bytes)
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );
    pb
}

#[derive(Clone, Debug)]
pub struct ConsumerFactsProfile {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub date_of_birth: NaiveDate,
    pub email: String,
    pub phone_number: String,
}

#[derive(Clone, Debug)]
pub struct CreditFactsProfile {
    pub application_datetime: NaiveDateTime,
    pub originated_datetime: Option<NaiveDateTime>,
    pub payment_due_date: Option<NaiveDateTime>,
    pub payment_due_amount: Option<f64>,
    pub credit_state: String,
}

#[derive(Clone)]
pub struct NamePair {
    pub id: i32,
    pub first: &'static str,
    pub last: &'static str,
}

pub fn generate_random_institution_names(rng: &mut impl Rng) -> Option<String> {
    let institutions = [
        "CIBC",
        "RBC",
        "TD",
        "Scotiabank",
        "BMO",
        "National Bank",
        "HSBC",
        "Desjardins",
        "ATB Financial",
        "Manulife Bank",
    ];
    institutions.choose(rng).map(|s| s.to_string())
}

pub fn generate_address(rng: &mut impl rand::Rng) -> String {
    let street: String = StreetName(EN).fake_with_rng(rng);
    let building_number: String = BuildingNumber(EN).fake_with_rng(rng);
    let city: String = CityName(EN).fake_with_rng(rng);
    let state: String = StateName(EN).fake_with_rng(rng);
    let zip_code: String = PostCode(EN).fake_with_rng(rng);

    format!(
        "{} {} {} {} {} {}",
        street, building_number, city, state, zip_code, "USA"
    )
}

pub fn generate_birthdate(hash: u64) -> NaiveDate {
    let year = 1970 + ((hash >> 3) % 30) as i32; // 1970..2000
    let month = 1 + ((hash >> 5) % 12) as u32; // 1..12
    let day = 1 + ((hash >> 7) % 27) as u32; // 1..28
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

pub fn generate_phone_number(rng: &mut impl rand::Rng) -> String {
    let local_number: String = NumberWithFormat("##########").fake_with_rng(rng); // 10 digits
    format!("+1{}", local_number)
}

pub fn generate_consumer_facts(name: &NamePair) -> ConsumerFactsProfile {
    let mut hasher = DefaultHasher::new();
    (name.first, name.last).hash(&mut hasher);
    let hash = hasher.finish();

    let mut local_rng = rand::rngs::StdRng::seed_from_u64(hash);

    ConsumerFactsProfile {
        first_name: name.first.to_string(),
        last_name: name.last.to_string(),
        address: generate_address(&mut local_rng),
        date_of_birth: generate_birthdate(hash),
        email: format!(
            "{}.{}@example.com",
            name.first.to_lowercase(),
            name.last.to_lowercase()
        ),
        phone_number: generate_phone_number(&mut local_rng),
    }
}

pub fn generate_credit_facts(state: &str, now: NaiveDateTime, amount: f64) -> CreditFactsProfile {
    let amt = Some(amount); // consistent dummy value
    let mut rng = rand::rng();

    // Create a normal distribution with mean=0 and std_dev=1
    let normal = Normal::new(0.0, 1.0).unwrap();

    // Helper function to add random variation to a duration
    // ~68% of variations will be within ±2 days
    let mut add_variation = |days: i64| -> i64 {
        let variation = normal.sample(&mut rng) * 2.0; // 2 days standard deviation
        (days as f64 + variation).round() as i64
    };

    match state {
        "application" => CreditFactsProfile {
            application_datetime: now - Duration::days(add_variation(10)),
            originated_datetime: None,
            payment_due_date: None,
            payment_due_amount: None,
            credit_state: "application".to_string(),
        },
        "originated" => {
            let application = now - Duration::days(add_variation(20));
            let originated = application + Duration::days(add_variation(1));
            let due = now + Duration::days(add_variation(14));

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amt,
                credit_state: "originated".to_string(),
            }
        }
        "declined" => CreditFactsProfile {
            application_datetime: now - Duration::days(add_variation(15)),
            originated_datetime: None,
            payment_due_date: None,
            payment_due_amount: None,
            credit_state: "declined".to_string(),
        },
        "non-compliant" => {
            let application = now - Duration::days(add_variation(60));
            let originated = application + Duration::days(add_variation(1));
            let due = originated + Duration::days(add_variation(30)); // but still in the past

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amt,
                credit_state: "non-compliant".to_string(),
            }
        }
        "compliant" => {
            let application = now - Duration::days(add_variation(40));
            let originated = application + Duration::days(add_variation(2));
            let due = originated + Duration::days(add_variation(25)); // in past but compliant

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amt,
                credit_state: "compliant".to_string(),
            }
        }
        _ => panic!("Unknown credit state"),
    }
}
