use chrono::{Duration, Local, NaiveDate};
use clap::{Parser, Subcommand};
use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};
use rand::seq::IndexedRandom;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

use klearlink_api::consumer_credit::models::InsertConsumerCreditModel;
use klearlink_api::schema::{consumer_credit, users};
use klearlink_api::seed_utils::{
    establish_connection, generate_consumer_facts, generate_credit_facts,
    generate_random_institution_names, generate_reproducible_uuid, progress_bar,
    ConsumerFactsProfile, NamePair,
};
use klearlink_api::user::models::{InsertUserModel, UserModel};

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
    /// Seed the database with the new consumer use case
    NewConsumer,
}

fn seed_fraud_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::thread_rng();

    let bnpl_1_user_id = 5;
    let bnpl_2_user_id = 6;

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
        institution_names: generate_random_institution_names(&mut rng),
        amount: 250.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(30),
        originated_datetime: Some(now - Duration::days(29)),
        payment_due_date: Some(now - Duration::days(15)),
        payment_due_amount: Some(250.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(2),
        paid_installments: Some(2),
        installment_amount: Some(125.0),
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
        institution_names: generate_random_institution_names(&mut rng),
        amount: 1500.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(20),
        originated_datetime: Some(now - Duration::days(19)),
        payment_due_date: Some(now - Duration::days(5)),
        payment_due_amount: Some(1075.0),
        credit_state: "non-compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(3),
        paid_installments: Some(1),
        installment_amount: Some(500.0),
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
        institution_names: generate_random_institution_names(&mut rng),
        amount: 1200.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(15),
        originated_datetime: Some(now - Duration::days(14)),
        payment_due_date: Some(now),
        payment_due_amount: Some(900.0),
        credit_state: "non-compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(1),
        installment_amount: Some(300.0),
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
        institution_names: generate_random_institution_names(&mut rng),
        amount: 250.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(10),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_2_user_id,
        total_installments: Some(2),
        paid_installments: Some(0),
        installment_amount: Some(125.0),
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

fn seed_new_consumer_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::thread_rng();

    let bnpl_1_user_id = 5;
    let bnpl_2_user_id = 6;

    // Create the base consumer profile
    let base_profile = ConsumerFactsProfile {
        first_name: "Sarah".to_string(),
        last_name: "Johnson".to_string(),
        address: "456 Oak Ave, Vancouver, BC, V6B 1H2".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1992, 5, 15).unwrap(),
        email: "sarah.johnson@example.com".to_string(),
        phone_number: "+16045551234".to_string(),
    };

    // First transaction - Old BNPL loan, fully paid
    let first_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 400.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(180),
        originated_datetime: Some(now - Duration::days(179)),
        payment_due_date: Some(now - Duration::days(150)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(4),
        installment_amount: Some(100.0),
    };

    // Second transaction - Old BNPL loan, fully paid
    let second_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 600.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(150),
        originated_datetime: Some(now - Duration::days(149)),
        payment_due_date: Some(now - Duration::days(120)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(3),
        paid_installments: Some(3),
        installment_amount: Some(200.0),
    };

    // Third transaction - Old BNPL loan, fully paid
    let third_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 800.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(120),
        originated_datetime: Some(now - Duration::days(119)),
        payment_due_date: Some(now - Duration::days(90)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(4),
        installment_amount: Some(200.0),
    };

    // Fourth transaction - Old BNPL loan, fully paid
    let fourth_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 500.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(90),
        originated_datetime: Some(now - Duration::days(89)),
        payment_due_date: Some(now - Duration::days(60)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(2),
        paid_installments: Some(2),
        installment_amount: Some(250.0),
    };

    // Fifth transaction - Active BNPL loan with one payment remaining
    let fifth_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 400.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(60),
        originated_datetime: Some(now - Duration::days(59)),
        payment_due_date: Some(now + Duration::days(1)),
        payment_due_amount: Some(100.0),
        credit_state: "originated".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(3),
        installment_amount: Some(100.0),
    };

    // Sixth transaction - New application with BNPL2
    let sixth_transaction = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_names: generate_random_institution_names(&mut rng),
        amount: 600.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(1),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        user_id: bnpl_2_user_id,
        total_installments: Some(3),
        paid_installments: Some(0),
        installment_amount: Some(200.0),
    };

    // Insert all transactions
    insert_into(consumer_credit::table)
        .values(&first_transaction)
        .execute(conn)
        .expect("Error inserting first new consumer transaction");

    insert_into(consumer_credit::table)
        .values(&second_transaction)
        .execute(conn)
        .expect("Error inserting second new consumer transaction");

    insert_into(consumer_credit::table)
        .values(&third_transaction)
        .execute(conn)
        .expect("Error inserting third new consumer transaction");

    insert_into(consumer_credit::table)
        .values(&fourth_transaction)
        .execute(conn)
        .expect("Error inserting fourth new consumer transaction");

    insert_into(consumer_credit::table)
        .values(&fifth_transaction)
        .execute(conn)
        .expect("Error inserting fifth new consumer transaction");

    insert_into(consumer_credit::table)
        .values(&sixth_transaction)
        .execute(conn)
        .expect("Error inserting sixth new consumer transaction");
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
        Commands::NewConsumer => {
            println!("Seeding database with new consumer use case...");
            seed_new_consumer_use_case(&mut establish_connection());
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
                total_installments: None,
                paid_installments: None,
                installment_amount: None,
            };

            insert_into(consumer_credit::table)
                .values(&consumer_credit)
                .execute(&mut conn)
                .expect("Error inserting consumer credit record");
        }
    }
    pb.finish_with_message("Database seeded with random data");
}
