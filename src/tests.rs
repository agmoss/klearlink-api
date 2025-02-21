#[cfg(test)]
mod tests {
    use crate::rocket;
    use once_cell::sync::Lazy;
    use rocket::{
        http::{ContentType, Header, Status},
        local::blocking::Client,
    };
    use serde_json::json;
    use serial_test::serial;
    use uuid::Uuid;

    static TEST_UUID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

    static API_KEY_1: &str = "c491a813-234a-4bea-b6c4-7413b244dea5";
    static API_KEY_2: &str = "c491a813-234a-4bea-b6c4-7413b244dea6";

    fn setup_test_users(client: &Client) {
        let users = vec![
            json!({
                "username": "test_user_1",
                "api_key": API_KEY_1,
                "role": "lender"
            }),
            json!({
                "username": "test_user_2",
                "api_key": API_KEY_2,
                "role": "lender"
            }),
        ];

        for user in users {
            let response = client
                .post("/users")
                .header(ContentType::JSON)
                .body(user.to_string())
                .dispatch();

            assert_eq!(response.status(), Status::Ok);
        }
    }

    #[test]
    #[serial]
    fn test_create_user() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(
                json!({
                    "username": Uuid::new_v4(),
                    "api_key": Uuid::new_v4()
                })
                .to_string(),
            )
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

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
            .header(Header::new("X-API-Key", API_KEY_1))
            .header(Header::new("X-Username", "test_user_1"))
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
            .header(Header::new("X-API-Key", API_KEY_1))
            .header(Header::new("X-Username", "test_user_1"))
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
            .header(Header::new("X-API-Key", API_KEY_1))
            .header(Header::new("X-Username", "test_user_1"))
            .body(missing_fields_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    #[serial]
    fn test_view_consumer_match_with_matches() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let test_uuid_matches = Uuid::new_v4().to_string();

        let test_payloads = vec![
            json!({
                "consumer_facts": {
                    "first_name": "John",
                    "last_name": "Doe",
                    "email": "john.doe@example.com",
                    "date_of_birth": "1990-01-01",
                    "address": "123 Test St",
                    "phone_number": "+1234567890",
                    "institution_names": ["Bank A"]
                },
                "credit_facts": {
                    "amount": 1000.0,
                    "credit_type": "PDL",
                    "application_datetime": "2024-01-01T12:00:00",
                    "credit_state": "application"
                }
            }),
            json!({
                "consumer_facts": {
                    "first_name": "John",
                    "last_name": "Doe",
                    "email": "john.doe@example.com",
                    "date_of_birth": "1990-01-01",
                    "address": "123 Test St",
                    "phone_number": "+1234567890",
                    "institution_names": ["Bank B"]
                },
                "credit_facts": {
                    "amount": 1500.0,
                    "credit_type": "PDL",
                    "application_datetime": "2024-01-02T12:00:00",
                    "credit_state": "application"
                }
            }),
        ];

        for (i, payload) in test_payloads.iter().enumerate() {
            let response = client
                .put(format!("/consumer-credit/{}_{}", test_uuid_matches, i + 1))
                .header(Header::new("X-API-Key", API_KEY_2))
                .header(Header::new("X-Username", format!("test_user_2")))
                .body(payload.to_string())
                .dispatch();
            assert_eq!(response.status(), Status::Ok);
        }

        let response = client
            .get(format!("/consumer-credit/{}/consumer-match", *TEST_UUID))
            .header(Header::new("X-Username", "test_user_1"))
            .header(Header::new("X-API-Key", API_KEY_1))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
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
            .header(Header::new("X-API-Key", API_KEY_1))
            .header(Header::new("X-Username", "test_user_1"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Second insertion with the same UUID should fail
        let response = client
            .put(format!("/consumer-credit/{}", test_uuid_duplicate))
            .header(Header::new("X-API-Key", API_KEY_1))
            .header(Header::new("X-Username", "test_user_1"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Conflict);
    }
}
