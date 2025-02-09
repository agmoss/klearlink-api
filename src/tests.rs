#[cfg(test)]
mod tests {
    use crate::{consumer_credit::models::InsertConsumerCredit, rocket};
    use once_cell::sync::Lazy;
    use rocket::{
        http::{Header, Status},
        local::blocking::Client,
    };
    use serde_json::json;
    use serial_test::serial;
    use uuid::Uuid;

    static TEST_UUID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

    #[test]
    #[serial]
    fn test_submit_consumer_credit() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let dummy_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "date_of_birth": "1990-01-01",
                "address": "123 Test St, Test City",
                "phone_number": "123-456-7890",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": 1000.0,
                "credit_type": "PDL",
                "application_datetime": "2024-01-01T12:00:00",
                "credit_state": "application"
            }
        });

        let response = client
            .put(format!("/consumer-credit/{}", *TEST_UUID))
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    #[serial]
    fn test_submit_consumer_credit_invalid_data() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let invalid_payload = json!({
            "consumer_facts": {
                "first_name": "",
                "last_name": "Doe",
                "email": "not-an-email",
                "date_of_birth": "invalid-date",
                "address": "123 Test St, Test City",
                "phone_number": "123-456-7890",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": -1000.0,
                "credit_type": "PDL",
                "application_datetime": "invalid-datetime",
                "credit_state": "application"
            }
        });

        let response = client
            .put(format!("/consumer-credit/{}", *TEST_UUID))
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(invalid_payload.to_string())
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    #[serial]
    fn test_submit_consumer_credit_missing_fields() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let missing_fields_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe"
            },
            "credit_facts": {
                "amount": 1000.0
            }
        });

        let response = client
            .put(format!("/consumer-credit/{}", *TEST_UUID))
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(missing_fields_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    #[serial]
    fn test_view_consumer_match_with_matches() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let connection = &mut establish_connection_pg();

        // Insert test records
        let test_records = vec![
            InsertConsumerCredit {
                consumer_credit_id: "test1".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
                address: "123 Test St".to_string(),
                phone_number: "+1234567890".to_string(),
                sin_ssn: None,
                institution_names: vec!["Bank A".to_string()],
            },
            InsertConsumerCredit {
                consumer_credit_id: "test2".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                date_of_birth: NaiveDate::from_ymd(1990, 1, 1),
                address: "123 Test St".to_string(),
                phone_number: "+1234567890".to_string(),
                sin_ssn: None,
                institution_names: vec!["Bank B".to_string()],
            },
        ];

        for record in test_records {
            diesel::insert_into(crate::schema::consumer_credit::table)
                .values(&record)
                .execute(connection)
                .expect("Error inserting test record");
        }

        // Call the view_consumer_match endpoint
        let response = client
            .get("/consumer-credit/test1/consumer-match")
            .header(Header::new("X-Username", "test_user"))
            .header(Header::new("X-API-Key", "test_key"))
            .dispatch();

        // Verify the response
        assert_eq!(response.status(), Status::Ok);
        let matches: Vec<ConsumerCreditDto> = response.into_json().expect("valid JSON response");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    #[serial]
    fn test_duplicate_consumer_credit_insertion() {
        let test_uuid_duplicate = Uuid::new_v4().to_string();
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let dummy_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "date_of_birth": "1990-01-01",
                "address": "123 Test St, Test City",
                "phone_number": "123-456-7890",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": 1000.0,
                "credit_type": "PDL",
                "application_datetime": "2024-01-01T12:00:00",
                "credit_state": "application"
            }
        });

        // First insertion should succeed
        let response = client
            .put(format!("/consumer-credit/{}", test_uuid_duplicate))
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Second insertion with the same UUID should fail
        let response = client
            .put(format!("/consumer-credit/{}", test_uuid_duplicate))
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Conflict);
    }
}
