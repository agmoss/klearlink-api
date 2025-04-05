use diesel::prelude::*;

use dotenv::dotenv;
use rand::seq::IndexedRandom;
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{Local, NaiveDate};
use uuid::Uuid;

use bigdecimal::{BigDecimal, FromPrimitive};
use klearlink_api::consumer_credit::models::InsertConsumerCreditModel;
use klearlink_api::schema::{consumer_credit, users};
use klearlink_api::user::models::{InsertUserModel, UserModel};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    println!("Seeding database with test data...");
    seed_database();
    println!("Database seeded successfully.");
}
fn seed_database() {
    let mut connn = establish_connection();

    use diesel::insert_into;
    use diesel::RunQueryDsl;
    use rand::Rng;

    let first_names = ["John", "Jane", "Alice", "Bob", "Charlie", "Diana"];
    let last_names = ["Doe", "Smith", "Johnson", "Lee", "Brown", "Davis"];
    let addresses = [
        "101 Main St",
        "202 Elm St",
        "303 Oak Ave",
        "404 Maple Rd",
        "505 Pine St",
        "606 Birch Blvd",
        "707 Cedar Ln",
        "808 Walnut Way",
        "909 Spruce Ct",
        "1000 Fir Dr",
        "1111 Aspen Ave",
        "1212 Cherry Blvd",
        "1313 Maple Way",
        "1414 Oak Ct",
        "1515 Pine Cir",
        "1616 Elm St",
        "1717 Cedar Dr",
        "1818 Birch Ct",
        "1919 Spruce St",
        "2020 Fir Ln",
        "2121 Aspen Ct",
        "2222 Cherry St",
        "2323 Maple Cir",
        "2424 Oak Dr",
        "2525 Pine Ave",
        "2626 Elm Blvd",
        "2727 Cedar Cir",
        "2828 Birch Ln",
        "2929 Spruce Ave",
        "3030 Fir Way",
        "3131 Aspen Dr",
        "3232 Cherry Ln",
        "3333 Maple Blvd",
        "3434 Oak St",
        "3535 Pine Way",
        "3636 Elm Cir",
    ]; // Make sure this covers all combinations

    let credit_types = ["PDL", "BNPL"];
    let credit_states = [
        "application",
        "originated",
        "declined",
        "non-compliant",
        "compliant",
    ];

    // Build deterministic mapping of (first_name, last_name) -> address
    let mut name_address_map: HashMap<(String, String), String> = HashMap::new();
    let mut used_addresses = addresses.to_vec();

    for first in &first_names {
        for last in &last_names {
            if used_addresses.is_empty() {
                panic!("Not enough unique addresses for name combinations");
            }

            let mut hasher = DefaultHasher::new();
            (first, last).hash(&mut hasher);
            let index = (hasher.finish() as usize) % used_addresses.len();
            let address = used_addresses.swap_remove(index);

            name_address_map.insert((first.to_string(), last.to_string()), address.to_string());
        }
    }

    for i in 0..10 {
        let new_user = InsertUserModel {
            username: format!("lender_{}", i),
            api_key: Uuid::new_v4(),
            role: "lender".to_string(),
        };

        let inserted_user = insert_into(users::table)
            .values(&new_user)
            .get_result::<UserModel>(&mut connn)
            .expect("Error inserting new user");

        for j in 0..10 {
            let cc_id = format!("cc_{}_{}", i, j);
            let now = Local::now().naive_local();
            let mut rng = rand::rng();

            let first_name = first_names.choose(&mut rng).unwrap().to_string();
            let last_name = last_names.choose(&mut rng).unwrap().to_string();
            let address = name_address_map
                .get(&(first_name.clone(), last_name.clone()))
                .unwrap()
                .clone();

            let credit_type = credit_types.choose(&mut rng).unwrap().to_string();
            let credit_state = credit_states.choose(&mut rng).unwrap().to_string();
            let amount: f64 = rng.random_range(500.0..2000.0);
            let dob_year = rng.random_range(1970..2000);
            let dob_month = rng.random_range(1..13);
            let dob_day = rng.random_range(1..28);
            let new_credit = InsertConsumerCreditModel {
                consumer_credit_id: cc_id,
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                email: format!("{}.{}{}@example.com", first_name, last_name, j),
                date_of_birth: NaiveDate::from_ymd_opt(dob_year, dob_month, dob_day).unwrap(),
                address,
                phone_number: "+11234567890".to_string(),
                sin_ssn: None,
                institution_names: vec![Some("TD".to_string())],
                amount: BigDecimal::from_f64(amount).unwrap(),
                credit_type,
                application_datetime: now,
                originated_datetime: None,
                payment_due_date: None,
                payment_due_amount: None,
                credit_state,
                consumer_information_indicator: None,
                user_id: inserted_user.id,
            };

            insert_into(consumer_credit::table)
                .values(&new_credit)
                .execute(&mut connn)
                .expect("Error inserting consumer credit record");
        }
    }
}
