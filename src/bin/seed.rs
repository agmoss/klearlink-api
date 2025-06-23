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
    /// Seed the database with the new consumer use case
    /// Use Case 1: New Customer TTV Optimization
    /// User: sarah.johnson@example.com
    NewConsumer,
    /// Seed the database with the bust out fraud use case
    /// Use Case 2: Bust Out Fraud
    /// User: michael.anderson@example.com
    BustOutFraud,
    /// Seed the database with the chargeoff risk use case
    /// Use Case 3: Reduce Chargeoff Risk with Real-Time Decisioning Data
    /// User: david.wilson@example.com
    ChargeoffRisk,
    /// Seed the database with the existing customer chargeoff risk use case
    /// Use Case 4: Chargeoff Risk Mitigation with Existing Customers
    /// User: james.miller@example.com
    ExistingCustomerRisk,
}

//Use Case 3: New Customer TTV Optimization
fn seed_new_consumer_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::rng();

    let bnpl_1_user_id = 3;
    let bnpl_2_user_id = 4;

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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 400.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(180),
        originated_datetime: Some(now - Duration::days(179)),
        payment_due_date: Some(now - Duration::days(150)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 600.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(150),
        originated_datetime: Some(now - Duration::days(149)),
        payment_due_date: Some(now - Duration::days(120)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 800.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(120),
        originated_datetime: Some(now - Duration::days(119)),
        payment_due_date: Some(now - Duration::days(90)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 500.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(90),
        originated_datetime: Some(now - Duration::days(89)),
        payment_due_date: Some(now - Duration::days(60)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 400.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(60),
        originated_datetime: Some(now - Duration::days(59)),
        payment_due_date: Some(now + Duration::days(1)),
        payment_due_amount: Some(100.0),
        credit_state: "originated".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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
        institution_name: generate_random_institution_names(&mut rng),
        amount: 600.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(1),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("10.0.0.1".to_string()),
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

fn seed_bust_out_fraud_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::rng();

    let bnpl_1_user_id = 3;
    let bnpl_2_user_id = 4;
    let bnpl_3_user_id = 5;
    let bnpl_4_user_id = 6;
    let bnpl_5_user_id = 7;

    // Create the base consumer profile for the fraudster
    let base_profile = ConsumerFactsProfile {
        first_name: "Michael".to_string(),
        last_name: "Anderson".to_string(),
        address: "789 Pine St, Montreal, QC, H3B 2Y7".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1988, 8, 15).unwrap(),
        email: "michael.anderson@example.com".to_string(),
        phone_number: "+15145551234".to_string(),
    };

    // First wave of BNPL tradelines (4 tradelines within 7 days)
    let first_wave = vec![
        // BNPL 1 - $250
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 250.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(60),
            originated_datetime: Some(now - Duration::days(59)),
            payment_due_date: Some(now - Duration::days(45)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.1".to_string()),
            user_id: bnpl_1_user_id,
            total_installments: Some(2),
            paid_installments: Some(2),
            installment_amount: Some(125.0),
        },
        // BNPL 2 - $300
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 300.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(58),
            originated_datetime: Some(now - Duration::days(57)),
            payment_due_date: Some(now - Duration::days(43)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.2".to_string()),
            user_id: bnpl_2_user_id,
            total_installments: Some(3),
            paid_installments: Some(3),
            installment_amount: Some(100.0),
        },
        // BNPL 3 - $250
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 250.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(56),
            originated_datetime: Some(now - Duration::days(55)),
            payment_due_date: Some(now - Duration::days(41)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.3".to_string()),
            user_id: bnpl_3_user_id,
            total_installments: Some(2),
            paid_installments: Some(2),
            installment_amount: Some(125.0),
        },
        // BNPL 4 - $200
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 200.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(54),
            originated_datetime: Some(now - Duration::days(53)),
            payment_due_date: Some(now - Duration::days(39)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.4".to_string()),
            user_id: bnpl_4_user_id,
            total_installments: Some(2),
            paid_installments: Some(2),
            installment_amount: Some(100.0),
        },
    ];

    // Second wave of BNPL tradelines (4 tradelines with same providers)
    let second_wave = vec![
        // BNPL 1 - $700
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 700.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(30),
            originated_datetime: Some(now - Duration::days(29)),
            payment_due_date: Some(now - Duration::days(15)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.5".to_string()),
            user_id: bnpl_1_user_id,
            total_installments: Some(4),
            paid_installments: Some(4),
            installment_amount: Some(175.0),
        },
        // BNPL 2 - $800
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 800.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(28),
            originated_datetime: Some(now - Duration::days(27)),
            payment_due_date: Some(now - Duration::days(13)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.6".to_string()),
            user_id: bnpl_2_user_id,
            total_installments: Some(4),
            paid_installments: Some(4),
            installment_amount: Some(200.0),
        },
        // BNPL 3 - $600
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 600.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(26),
            originated_datetime: Some(now - Duration::days(25)),
            payment_due_date: Some(now - Duration::days(11)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.7".to_string()),
            user_id: bnpl_3_user_id,
            total_installments: Some(3),
            paid_installments: Some(3),
            installment_amount: Some(200.0),
        },
        // BNPL 4 - $400
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 400.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(24),
            originated_datetime: Some(now - Duration::days(23)),
            payment_due_date: Some(now - Duration::days(9)),
            payment_due_amount: Some(0.0),
            credit_state: "compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.8".to_string()),
            user_id: bnpl_4_user_id,
            total_installments: Some(2),
            paid_installments: Some(2),
            installment_amount: Some(200.0),
        },
    ];

    // Final application to a new provider
    let final_application = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 300.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(1),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.9".to_string()),
        user_id: bnpl_5_user_id,
        total_installments: Some(2),
        paid_installments: Some(0),
        installment_amount: Some(150.0),
    };

    // Insert all transactions
    for transaction in first_wave {
        insert_into(consumer_credit::table)
            .values(&transaction)
            .execute(conn)
            .expect("Error inserting first wave transaction");
    }

    for transaction in second_wave {
        insert_into(consumer_credit::table)
            .values(&transaction)
            .execute(conn)
            .expect("Error inserting second wave transaction");
    }

    insert_into(consumer_credit::table)
        .values(&final_application)
        .execute(conn)
        .expect("Error inserting final application");
}

fn seed_chargeoff_risk_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::rng();

    // Create the base consumer profile
    let base_profile = ConsumerFactsProfile {
        first_name: "David".to_string(),
        last_name: "Wilson".to_string(),
        address: "123 Maple Dr, Toronto, ON, M5V 2T6".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1995, 3, 20).unwrap(),
        email: "david.wilson@example.com".to_string(),
        phone_number: "+14165551234".to_string(),
    };

    let bnpl_1_user_id = 3;
    let bnpl_2_user_id = 4;
    let bnpl_3_user_id = 5;
    let bnpl_4_user_id = 6;

    // First BNPL tradeline - Non-compliant
    let first_tradeline = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 100.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(21),
        originated_datetime: Some(now - Duration::days(20)),
        payment_due_date: Some(now - Duration::days(6)),
        payment_due_amount: Some(50.0),
        credit_state: "non-compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.10".to_string()),
        user_id: bnpl_1_user_id,
        total_installments: Some(2),
        paid_installments: Some(1),
        installment_amount: Some(50.0),
    };

    // First declined application (24 hours ago)
    let first_declined = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 150.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(1),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "declined".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.11".to_string()),
        user_id: bnpl_2_user_id,
        total_installments: Some(3),
        paid_installments: Some(0),
        installment_amount: Some(50.0),
    };

    // Second declined application (12 hours ago)
    let second_declined = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 200.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::hours(12),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "declined".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.12".to_string()),
        user_id: bnpl_3_user_id,
        total_installments: Some(4),
        paid_installments: Some(0),
        installment_amount: Some(50.0),
    };

    // Current application (new)
    let current_application = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 200.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now,
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.13".to_string()),
        user_id: bnpl_4_user_id,
        total_installments: Some(4),
        paid_installments: Some(0),
        installment_amount: Some(50.0),
    };

    // Insert all transactions
    insert_into(consumer_credit::table)
        .values(&first_tradeline)
        .execute(conn)
        .expect("Error inserting first tradeline");

    insert_into(consumer_credit::table)
        .values(&first_declined)
        .execute(conn)
        .expect("Error inserting first declined application");

    insert_into(consumer_credit::table)
        .values(&second_declined)
        .execute(conn)
        .expect("Error inserting second declined application");

    insert_into(consumer_credit::table)
        .values(&current_application)
        .execute(conn)
        .expect("Error inserting current application");
}

fn seed_existing_customer_risk_use_case(conn: &mut PgConnection) {
    let now = Local::now().naive_local();
    let mut rng = rand::rng();

    // Create the base consumer profile
    let base_profile = ConsumerFactsProfile {
        first_name: "James".to_string(),
        last_name: "Miller".to_string(),
        address: "789 Yonge St, Toronto, ON, M4W 2G8".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1987, 11, 30).unwrap(),
        email: "james.miller@example.com".to_string(),
        phone_number: "+14165553456".to_string(),
    };

    let bnpl_1_user_id = 3;
    let bnpl_2_user_id = 4;
    let bnpl_3_user_id = 5;
    let bnpl_4_user_id = 6;
    let bnpl_5_user_id = 7;

    // First BNPL1 transaction - Completed successfully
    let first_bnpl1 = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 500.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(60),
        originated_datetime: Some(now - Duration::days(59)),
        payment_due_date: Some(now - Duration::days(15)),
        payment_due_amount: Some(0.0),
        credit_state: "compliant".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.20".to_string()),
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(4),
        installment_amount: Some(125.0),
    };

    // Three non-compliant BNPL tradelines from other providers (last 4 weeks)
    let other_bnpl_tradelines = vec![
        // BNPL2 - Non-compliant
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 200.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(28),
            originated_datetime: Some(now - Duration::days(27)),
            payment_due_date: Some(now - Duration::days(13)),
            payment_due_amount: Some(100.0),
            credit_state: "non-compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.20".to_string()),
            user_id: bnpl_2_user_id,
            total_installments: Some(2),
            paid_installments: Some(1),
            installment_amount: Some(100.0),
        },
        // BNPL3 - Non-compliant
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 200.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(21),
            originated_datetime: Some(now - Duration::days(20)),
            payment_due_date: Some(now - Duration::days(6)),
            payment_due_amount: Some(100.0),
            credit_state: "non-compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.20".to_string()),
            user_id: bnpl_3_user_id,
            total_installments: Some(2),
            paid_installments: Some(1),
            installment_amount: Some(100.0),
        },
        // BNPL4 - Non-compliant
        InsertConsumerCreditModel {
            consumer_credit_id: Uuid::new_v4().to_string(),
            first_name: base_profile.first_name.clone(),
            last_name: base_profile.last_name.clone(),
            email: base_profile.email.clone(),
            date_of_birth: base_profile.date_of_birth,
            address: base_profile.address.clone(),
            phone_number: base_profile.phone_number.clone(),
            sin_ssn: None,
            institution_name: generate_random_institution_names(&mut rng),
            amount: 200.0,
            credit_type: "BNPL".to_string(),
            application_datetime: now - Duration::days(14),
            originated_datetime: Some(now - Duration::days(13)),
            payment_due_date: Some(now + Duration::days(1)),
            payment_due_amount: Some(100.0),
            credit_state: "non-compliant".to_string(),
            consumer_information_indicator: None,
            ip_address: Some("192.168.1.20".to_string()),
            user_id: bnpl_4_user_id,
            total_installments: Some(2),
            paid_installments: Some(1),
            installment_amount: Some(100.0),
        },
    ];

    // Recent declined application
    let declined_application = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 300.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now - Duration::days(5),
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "declined".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.20".to_string()),
        user_id: bnpl_5_user_id,
        total_installments: Some(3),
        paid_installments: Some(0),
        installment_amount: Some(100.0),
    };

    // Current application to BNPL1
    let current_application = InsertConsumerCreditModel {
        consumer_credit_id: Uuid::new_v4().to_string(),
        first_name: base_profile.first_name.clone(),
        last_name: base_profile.last_name.clone(),
        email: base_profile.email.clone(),
        date_of_birth: base_profile.date_of_birth,
        address: base_profile.address.clone(),
        phone_number: base_profile.phone_number.clone(),
        sin_ssn: None,
        institution_name: generate_random_institution_names(&mut rng),
        amount: 1000.0,
        credit_type: "BNPL".to_string(),
        application_datetime: now,
        originated_datetime: None,
        payment_due_date: None,
        payment_due_amount: None,
        credit_state: "application".to_string(),
        consumer_information_indicator: None,
        ip_address: Some("192.168.1.20".to_string()),
        user_id: bnpl_1_user_id,
        total_installments: Some(4),
        paid_installments: Some(0),
        installment_amount: Some(250.0),
    };

    // Insert all transactions
    insert_into(consumer_credit::table)
        .values(&first_bnpl1)
        .execute(conn)
        .expect("Error inserting first BNPL1 transaction");

    for tradeline in other_bnpl_tradelines {
        insert_into(consumer_credit::table)
            .values(&tradeline)
            .execute(conn)
            .expect("Error inserting other BNPL tradeline");
    }

    insert_into(consumer_credit::table)
        .values(&declined_application)
        .execute(conn)
        .expect("Error inserting declined application");

    insert_into(consumer_credit::table)
        .values(&current_application)
        .execute(conn)
        .expect("Error inserting current application");
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random => {
            println!("Seeding database with random data...");
            seed_random_data();
        }
        Commands::NewConsumer => {
            println!("Seeding database with new consumer use case...");
            seed_new_consumer_use_case(&mut establish_connection());
        }
        Commands::BustOutFraud => {
            println!("Seeding database with bust out fraud use case...");
            seed_bust_out_fraud_use_case(&mut establish_connection());
        }
        Commands::ChargeoffRisk => {
            println!("Seeding database with chargeoff risk use case...");
            seed_chargeoff_risk_use_case(&mut establish_connection());
        }
        Commands::ExistingCustomerRisk => {
            println!("Seeding database with existing customer chargeoff risk use case...");
            seed_existing_customer_risk_use_case(&mut establish_connection());
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

    let lenders = 10;
    let lendees_per_lender = 15;

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
                institution_name: generate_random_institution_names(&mut rng),
                amount,
                credit_type: credit_types.choose(&mut rng).unwrap().to_string(),
                application_datetime: credit_facts.application_datetime,
                originated_datetime: credit_facts.originated_datetime,
                payment_due_date: credit_facts.payment_due_date,
                payment_due_amount: credit_facts.payment_due_amount,
                credit_state: credit_facts.credit_state,
                consumer_information_indicator: None,
                ip_address: Some(format!(
                    "172.16.{}.{}",
                    rng.random_range(0..255),
                    rng.random_range(0..255)
                )),
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
