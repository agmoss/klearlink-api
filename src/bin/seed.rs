use diesel::prelude::*;

use fake::Fake;

use dotenv::dotenv;
use rand::seq::IndexedRandom;
use rand::SeedableRng;
use std::collections::HashMap;
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use uuid::Uuid;

use klearlink_api::consumer_credit::models::InsertConsumerCreditModel;
use klearlink_api::schema::{consumer_credit, users};
use klearlink_api::user::models::{InsertUserModel, UserModel};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Clone, Debug)]
struct PersonProfile {
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

fn generate_profile(first: &str, last: &str) -> PersonProfile {
    let mut hasher = DefaultHasher::new();
    (first, last).hash(&mut hasher);
    let hash = hasher.finish();

    // Deterministic birthdate based on hash
    let year = 1970 + ((hash >> 3) % 30) as i32; // 1970..2000
    let month = 1 + ((hash >> 5) % 12) as u32; // 1..12
    let day = 1 + ((hash >> 7) % 27) as u32; // 1..28

    let dob = NaiveDate::from_ymd_opt(year, month, day).unwrap();

    // Deterministic address
    let mut local_rng = rand::rngs::StdRng::seed_from_u64(hash);

    use fake::faker::address::raw::*;
    use fake::faker::phone_number::raw::*;
    use fake::locales::*;
    let address: String = SecondaryAddress(EN).fake_with_rng(&mut local_rng);
    let _phone_number: String = PhoneNumber(EN).fake_with_rng(&mut local_rng);

    let email = format!(
        "{}.{}@example.com",
        first.to_lowercase(),
        last.to_lowercase()
    );

    PersonProfile {
        address,
        date_of_birth: dob,
        email,
        phone_number: "+11234567890".to_string(),
    }
}

fn generate_credit_facts(state: &str, now: NaiveDateTime, amount: f64) -> CreditFactsProfile {
    let amoun = Some(amount); // consistent dummy value

    match state {
        "application" => CreditFactsProfile {
            application_datetime: now - Duration::days(10),
            originated_datetime: None,
            payment_due_date: None,
            payment_due_amount: None,
            credit_state: "application".to_string(),
        },
        "originated" => {
            let application = now - Duration::days(20);
            let originated = application + Duration::days(1);
            let due = now + Duration::days(14);

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amoun,
                credit_state: "originated".to_string(),
            }
        }
        "declined" => CreditFactsProfile {
            application_datetime: now - Duration::days(15),
            originated_datetime: None,
            payment_due_date: None,
            payment_due_amount: None,
            credit_state: "declined".to_string(),
        },
        "non-compliant" => {
            let application = now - Duration::days(60);
            let originated = application + Duration::days(1);
            let due = originated + Duration::days(30); // but still in the past

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amoun,
                credit_state: "non-compliant".to_string(),
            }
        }
        "compliant" => {
            let application = now - Duration::days(40);
            let originated = application + Duration::days(2);
            let due = originated + Duration::days(25); // in past but compliant

            CreditFactsProfile {
                application_datetime: application,
                originated_datetime: Some(originated),
                payment_due_date: Some(due),
                payment_due_amount: amoun,
                credit_state: "compliant".to_string(),
            }
        }
        _ => panic!("Unknown credit state"),
    }
}

fn seed_database() {
    let mut connn = establish_connection();

    use diesel::insert_into;
    use diesel::RunQueryDsl;
    use rand::Rng;

    let first_names = ["John", "Jane", "Alice", "Bob", "Charlie", "Diana"];
    let last_names = ["Doe", "Smith", "Johnson", "Lee", "Brown", "Davis"];

    let credit_types = ["PDL", "BNPL"];
    let credit_states = [
        "application",
        "originated",
        "declined",
        "non-compliant",
        "compliant",
    ];

    let name_combinations: Vec<(String, String)> = first_names
        .iter()
        .flat_map(|&first| {
            last_names
                .iter()
                .map(move |&last| (first.to_string(), last.to_string()))
        })
        .collect();

    let profile_map: HashMap<(String, String), PersonProfile> = name_combinations
        .iter()
        .map(|(f, l)| {
            let profile = generate_profile(f, l);
            ((f.clone(), l.clone()), profile)
        })
        .collect();

    for i in 0..5 {
        let lending_user = InsertUserModel {
            username: format!("lender_{}", i),
            api_key: Uuid::new_v4(),
            role: "lender".to_string(),
        };

        let inserted_user = insert_into(users::table)
            .values(&lending_user)
            .get_result::<UserModel>(&mut connn)
            .expect("Error inserting new user");

        for j in 0..25 {
            let cc_id = format!("cc_{}_{}", i, j);
            let now = Local::now().naive_local();
            let mut rng = rand::rng();

            let first_name = first_names.choose(&mut rng).unwrap().to_string();
            let last_name = last_names.choose(&mut rng).unwrap().to_string();

            let profile = profile_map
                .get(&(first_name.clone(), last_name.clone()))
                .unwrap()
                .clone();

            let credit_type = credit_types.choose(&mut rng).unwrap().to_string();
            let amount: f64 = rng.random_range(500.0..2000.0);

            let credit_state_choice = credit_states.choose(&mut rng).unwrap();

            let credit_facts = generate_credit_facts(credit_state_choice, now, amount);

            let new_credit = InsertConsumerCreditModel {
                consumer_credit_id: cc_id,
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                email: profile.email,
                date_of_birth: profile.date_of_birth,
                address: profile.address,
                phone_number: profile.phone_number,
                sin_ssn: None,
                institution_names: vec![Some("TD".to_string())],
                amount: amount,
                credit_type,
                application_datetime: credit_facts.application_datetime,
                originated_datetime: credit_facts.originated_datetime,
                payment_due_date: credit_facts.payment_due_date,
                payment_due_amount: credit_facts.payment_due_amount,
                credit_state: credit_facts.credit_state,
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

fn main() {
    println!("Seeding database with test data...");
    seed_database();
    println!("Database seeded successfully.");
}
