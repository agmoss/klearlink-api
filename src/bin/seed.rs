use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use chrono::{NaiveDate, NaiveDateTime, Local};
use uuid::Uuid;

 // Import your schema modules
use klearlink_api::schema::{users, consumer_credit};
use klearlink_api::consumer_credit::models::InsertConsumerCreditModel;
use bigdecimal::BigDecimal;

// Make sure your Diesel table name annotations match your schema.
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    api_key: Uuid,
    role: String,
}


fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = diesel::pg::PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    println!("Seeding database with test data...");
    seed_database(&connection);
    println!("Database seeded successfully.");
}

fn seed_database(conn: &diesel::pg::PgConnection) {
    use diesel::insert_into;
    use diesel::RunQueryDsl;
    use rand::seq::SliceRandom;
    use rand::Rng;
    let first_names = ["John", "Jane", "Alice", "Bob", "Charlie", "Diana"];
    let last_names = ["Doe", "Smith", "Johnson", "Lee", "Brown", "Davis"];
    let addresses = ["101 Main St", "202 Elm St", "303 Oak Ave", "404 Maple Rd", "505 Pine St"];
    let credit_types = ["PDL", "BNPL"];
    let credit_states = ["application", "originated", "declined", "non-compliant", "compliant"];

    // Seed 1000 users
    for i in 0..1000 {
        let new_user = NewUser {
            username: format!("user_{}", i),
            api_key: Uuid::new_v4(),
            role: "user".to_string(),
        };

        // Insert new user and get its auto-generated id
        let inserted_user: (i32,) = insert_into(users::table)
            .values(&new_user)
            .returning(users::id)
            .get_result(conn)
            .expect("Error inserting new user");

        // For each user, insert 10 consumer credit records
        for j in 0..10 {
            let cc_id = format!("cc_{}_{}", i, j);
            let now = Local::now().naive_local();
            let mut rng = rand::thread_rng();
            let first_name = first_names.choose(&mut rng).unwrap().to_string();
            let last_name = last_names.choose(&mut rng).unwrap().to_string();
            let address = addresses.choose(&mut rng).unwrap().to_string();
            let credit_type = credit_types.choose(&mut rng).unwrap().to_string();
            let credit_state = credit_states.choose(&mut rng).unwrap().to_string();
            let amount: f64 = rng.gen_range(500.0..2000.0);
            let dob_year = rng.gen_range(1970..2000);
            let dob_month = rng.gen_range(1..13);
            let dob_day = rng.gen_range(1..28);
            let new_credit = InsertConsumerCreditModel {
                consumer_credit_id: cc_id,
                first_name: first_name.clone(),
                last_name: last_name.clone(),
                email: format!("{}.{}{}@example.com", first_name, last_name, j),
                date_of_birth: NaiveDate::from_ymd(dob_year, dob_month, dob_day),
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
                user_id: inserted_user.0,
            };

            insert_into(consumer_credit::table)
                .values(&new_credit)
                .execute(conn)
                .expect("Error inserting consumer credit record");
        }
    }
}
