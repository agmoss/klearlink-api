use diesel::prelude::*;

use dotenv::dotenv;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom;
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use diesel::{insert_into, RunQueryDsl};

use fake::faker::address::raw::*;
use fake::faker::number::en::NumberWithFormat;
use fake::locales::*;
use fake::Fake;

use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use uuid::Uuid;

use klearlink_api::consumer_credit::models::InsertConsumerCreditModel;
use klearlink_api::schema::{consumer_credit, users};
use klearlink_api::user::models::{InsertUserModel, UserModel};

use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Seed the database with random data
    Random,
    /// Seed the database with the fraud use case
    UseCase,
}

fn generate_reproducible_uuid(seed: u128) -> Uuid {
    // Use a fixed seed to generate a reproducible UUID
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let bytes: [u8; 16] = rng.random();
    Uuid::from_bytes(bytes)
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn progress_bar(len: u64) -> ProgressBar {
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
struct ConsumerFactsProfile {
    first_name: String,
    last_name: String,
    address: String,
    date_of_birth: NaiveDate,
    email: String,
    phone_number: String,
}

#[derive(Clone, Debug)]
struct CreditFactsProfile {
    application_datetime: NaiveDateTime,
    originated_datetime: Option<NaiveDateTime>,
    payment_due_date: Option<NaiveDateTime>,
    payment_due_amount: Option<f64>,
    credit_state: String,
}

#[derive(Clone)]
struct NamePair {
    id: i32,
    first: &'static str,
    last: &'static str,
}

fn generate_address(rng: &mut impl rand::Rng) -> String {
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

fn generate_birthdate(hash: u64) -> NaiveDate {
    let year = 1970 + ((hash >> 3) % 30) as i32; // 1970..2000
    let month = 1 + ((hash >> 5) % 12) as u32; // 1..12
    let day = 1 + ((hash >> 7) % 27) as u32; // 1..28
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

fn generate_phone_number(rng: &mut impl rand::Rng) -> String {
    let local_number: String = NumberWithFormat("##########").fake_with_rng(rng); // 10 digits
    format!("+1{}", local_number)
}

fn generate_random_institution_names(rng: &mut ThreadRng) -> Vec<Option<String>> {
    let available_banks = ["TD", "RBC", "Scotiabank", "BMO", "CIBC"];
    let subset_size = rng.random_range(1..=available_banks.len());
    available_banks
        .choose_multiple(rng, subset_size)
        .map(|&bank| Some(bank.to_string()))
        .collect()
}

fn generate_consumer_facts(name: &NamePair) -> ConsumerFactsProfile {
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

fn generate_credit_facts(state: &str, now: NaiveDateTime, amount: f64) -> CreditFactsProfile {
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

fn seed_fraud_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();

    let bnpl_1_user_id = 13;
    let bnpl_2_user_id = 14;

    // Create the base consumer profile that will be used for both legitimate and fraudulent accounts
    let base_profile = ConsumerFactsProfile {
        first_name: "Sam".to_string(),
        last_name: "Jones".to_string(),
        address: "123 Main St, Toronto, ON, M5V 3L9".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        email: "sam.jones@example.com".to_string(),
        phone_number: "+14155551234".to_string(),
    };

    // First transaction - Small ticket item with BNPL 1, repaid in full
    let first_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: vec![Some("BNPL1".to_string())],
        amount: 250.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(30),
        originated_datetime: Some(now - Duration::days(29)),
        payment_due_date: Some(now - Duration::days(15)),
        payment_due_amount: Some(250.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
    };

    // Second transaction - Couch with BNPL 1, partial payment
    let second_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: vec![Some("BNPL1".to_string())],
        amount: 1500.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(20),
        originated_datetime: Some(now - Duration::days(19)),
        payment_due_date: Some(now - Duration::days(5)),
        payment_due_amount: Some(1075.0),
        credit_state: "non-compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
    };

    // Third transaction - Duplicate account with BNPL 1
    let third_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: vec![Some("BNPL1".to_string())],
        amount: 1200.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(15),
        originated_datetime: Some(now - Duration::days(14)),
        payment_due_date: Some(now),
        payment_due_amount: Some(900.0),
        credit_state: "non-compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
    };

    // Fourth transaction - Attempt with BNPL 2 using same credentials
    let fourth_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: vec![Some("BNPL2".to_string())],
        amount: 250.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(10),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_2_user_id,
    };

    // Insert all transactions
    insert_into(consumer_credit::table)
        .values(&first_transaction)
        .execute(conn)
        .expect("Error inserting first fraud transaction");

    insert_into(consumer_credit::table)
        .values(&second_transaction)
        .execute(conn)
        .expect("Error inserting second fraud transaction");

    insert_into(consumer_credit::table)
        .values(&third_transaction)
        .execute(conn)
        .expect("Error inserting third fraud transaction");

    insert_into(consumer_credit::table)
        .values(&fourth_transaction)
        .execute(conn)
        .expect("Error inserting fourth fraud transaction");

}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random => {
            println!("Seeding database with random data...");
            seed_random_data();
        }
        Commands::UseCase => {
            println!("Seeding database with fraud use case...");
            seed_fraud_use_case(&mut establish_connection());
        }
    }
}

fn seed_random_data() {
    let mut conn = establish_connection();

    let names = [
        NamePair {
            id: 1,
            first: "John",
            last: "Doe",
        },
        NamePair {
            id: 2,
            first: "Jane",
            last: "Smith",
        },
        NamePair {
            id: 3,
            first: "Alice",
            last: "Johnson",
        },
        NamePair {
            id: 4,
            first: "Bob",
            last: "Lee",
        },
        NamePair {
            id: 5,
            first: "Charlie",
            last: "Brown",
        },
        NamePair {
            id: 6,
            first: "Diana",
            last: "Davis",
        },
        NamePair {
            id: 7,
            first: "Edward",
            last: "Wilson",
        },
        NamePair {
            id: 8,
            first: "Fiona",
            last: "Taylor",
        },
        NamePair {
            id: 9,
            first: "George",
            last: "Anderson",
        },
        NamePair {
            id: 10,
            first: "Helen",
            last: "Thomas",
        },
    ];

    let credit_types = ["PDL", "BNPL"];
    let credit_states = [
        "application",
        "originated",
        "declined",
        "non-compliant",
        "compliant",
    ];

    let consumer_facts_profile_map: HashMap<i32, ConsumerFactsProfile> = names
        .iter()
        .map(|name| (name.id, generate_consumer_facts(name)))
        .collect();

    let lenders = 5;
    let lendees_per_lender = 7;

    let pb = progress_bar(lenders * lendees_per_lender);
    for i in 0..lenders {
        let lending_user = InsertUserModel {
            username: format!("lender_{}", i),
            api_key: generate_reproducible_uuid(i as u128),
            role: "lender".to_string(),
        };

        let inserted_lending_user = insert_into(users::table)
            .values(&lending_user)
            .get_result::<UserModel>(&mut conn)
            .expect("Error inserting new user");

        for _j in 0..lendees_per_lender {
            pb.inc(1);
            let now = Local::now().naive_local();
            let mut rng = rand::rng();

            let name = names.choose(&mut rng).unwrap();
            let profile = consumer_facts_profile_map.get(&name.id).unwrap().clone();

            let amount: f64 = rng.random_range(500.0..2000.0);

            let credit_state_choice = credit_states.choose(&mut rng).unwrap();

            let credit_facts = generate_credit_facts(credit_state_choice, now, amount);

            let consumer_credit = InsertConsumerCreditModel {
                consumer_credit_id: Uuid::new_v4().to_string(),
                first_name: profile.first_name,
                last_name: profile.last_name,
                email: profile.email,
                date_of_birth: profile.date_of_birth,
                address: profile.address,
                phone_number: profile.phone_number,
                sin_ssn: None,
                institution_names: generate_random_institution_names(&mut rng),
                amount,
                credit_type: credit_types.choose(&mut rng).unwrap().to_string(),
                application_datetime: credit_facts.application_datetime,
                originated_datetime: credit_facts.originated_datetime,
                payment_due_date: credit_facts.payment_due_date,
                payment_due_amount: credit_facts.payment_due_amount,
                credit_state: credit_facts.credit_state,
                consumer_information_indicator: None,
                user_id: inserted_lending_user.id,
            };

            insert_into(consumer_credit::table)
                .values(&consumer_credit)
                .execute(&mut conn)
                .expect("Error inserting consumer credit record");
        }
    }
    pb.finish_with_message("Database seeded with random data");
}
