use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use chrono::{NaiveDate, NaiveDateTime, Local};
use uuid::Uuid;

// Import your schema modules
use klearlink_api::schema::{users, consumer_credit};

// Make sure your Diesel table name annotations match your schema.
#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    username: String,
    api_key: Uuid,
    role: String,
}

#[derive(Insertable)]
#[diesel(table_name = consumer_credit)]
struct NewConsumerCredit {
    consumer_credit_id: String,
    first_name: String,
    last_name: String,
    email: String,
    date_of_birth: NaiveDate,
    address: String,
    phone_number: String,
    sin_ssn: Option<String>,
    institution_names: Vec<String>,
    amount: f64,
    credit_type: String,
    application_datetime: NaiveDateTime,
    originated_datetime: Option<NaiveDateTime>,
    payment_due_date: Option<NaiveDateTime>,
    payment_due_amount: Option<f64>,
    credit_state: String,
    consumer_information_indicator: Option<String>,
    user_id: i32,
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
            let new_credit = NewConsumerCredit {
                consumer_credit_id: cc_id,
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                email: format!("user_{}@example.com", i),
                date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
                address: "101 Main St".to_string(),
                phone_number: "+11234567890".to_string(),
                sin_ssn: None,
                institution_names: vec!["TD".to_string()],
                amount: 1000.00,
                credit_type: "PDL".to_string(),
                application_datetime: now,
                originated_datetime: None,
                payment_due_date: None,
                payment_due_amount: None,
                credit_state: "application".to_string(),
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
