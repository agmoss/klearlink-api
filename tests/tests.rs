mod common;

#[cfg(test)]
mod tests {

    use once_cell::sync::Lazy;
    use rocket::http::Status;
    use serde_json::json;
    use serial_test::serial;
    use uuid::Uuid;

    use crate::common::{
        create_consumer_credit, create_user, delete_user, response_json_value, test_client,
        update_consumer_credit, view_consumer_match,
    };

    static TEST_UUID: Lazy<String> = Lazy::new(|| Uuid::new_v4().to_string());

    static API_KEY_ADMIN: &str = "c491a813-234a-4bea-b6c4-7413b244dea4";
    static API_KEY_1: &str = "c491a813-234a-4bea-b6c4-7413b244dea5";
    static API_KEY_2: &str = "c491a813-234a-4bea-b6c4-7413b244dea6";

    #[test]
    fn _global_setup() {
        let client = test_client().lock().unwrap();

        let users = vec![
            //json!({
            //    "username": "test_admin_user",
            //    "api_key": API_KEY_ADMIN,
            //    "role": "admin"
            // }),
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
            let response = create_user(&client, &user, API_KEY_ADMIN.to_string());

            assert_eq!(response.status(), Status::Ok);
        }

        println!("✅ Global Setup: Test users created.");
    }

    // #[test]
    fn _global_teardown() {
        let client = test_client().lock().unwrap();

        let usernames = vec!["test_admin_user", "test_user_1", "test_user_2"];
        for username in usernames {
            let r1 = client
                .delete(format!("/consumer-credit/user/{}", username))
                .dispatch();

            assert_eq!(r1.status(), Status::Ok);

            let r2 = delete_user(&client, username.to_string(), API_KEY_ADMIN.to_string());

            assert_eq!(r2.status(), Status::Ok);
        }

        println!("🧹 Global Cleanup: Test users deleted.");
    }

    #[test]
    #[serial]
    fn test_create_user() {
        let client = test_client().lock().unwrap();

        let new_user = json!({
            "username": Uuid::new_v4().to_string(),
            "api_key": Uuid::new_v4().to_string(),
            "role": "lender"
        });

        let response = create_user(&client, &new_user, API_KEY_ADMIN.to_string());

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    #[serial]
    fn test_delete_user() {
        let client = test_client().lock().unwrap();

        let response = delete_user(&client, "new_user".to_string(), API_KEY_ADMIN.to_string());

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    #[serial]
    fn test_submit_consumer_credit() {
        let client = test_client().lock().unwrap();

        let dummy_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "date_of_birth": "1990-01-01",
                "address": "123 Main St, Toronto, ON, M5V 3L9",
                "phone_number": "+11234567890",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": 1000.0,
                "credit_type": "PDL",
                "application_datetime": "2024-01-01T12:00:00",
                "credit_state": "application"
            }
        });

        let response =
            create_consumer_credit(&client, &*TEST_UUID, API_KEY_1.to_string(), &dummy_payload);

        assert_eq!(response.status(), Status::Ok);

        let value = response_json_value(response);

        let title = value
            .get("consumer_facts")
            .expect("must have an 'consumer_facts' field")
            .get("first_name")
            .expect("must have a 'first_name' field")
            .as_str();

        assert_eq!(title, Some("John"));
    }

    #[test]
    #[serial]
    fn test_submit_consumer_credit_invalid_data() {
        let client = test_client().lock().unwrap();

        let invalid_payload = json!({
            "consumer_facts": {
                "first_name": "",
                "last_name": "Doe",
                "email": "not-an-email",
                "date_of_birth": "invalid-date",
                "address": "123 Test St, Test City",
                "phone_number": "+11234567890",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": -1000.0,
                "credit_type": "PDL",
                "application_datetime": "invalid-datetime",
                "credit_state": "application"
            }
        });

        let response = create_consumer_credit(
            &client,
            &*TEST_UUID,
            API_KEY_1.to_string(),
            &invalid_payload,
        );

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    #[serial]
    fn test_submit_consumer_credit_missing_fields() {
        let client = test_client().lock().unwrap();

        let missing_fields_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe"
            },
            "credit_facts": {
                "amount": 1000.0
            }
        });

        let response = create_consumer_credit(
            &client,
            &*TEST_UUID,
            API_KEY_1.to_string(),
            &missing_fields_payload,
        );

        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    #[serial]
    fn test_view_consumer_match_with_matches() {
        let client = test_client().lock().unwrap();

        let test_uuid_matches = Uuid::new_v4().to_string();

        let test_payloads = vec![
            json!({
                "consumer_facts": {
                    "first_name": "John",
                    "last_name": "Doe",
                    "email": "john.doe@example.com",
                    "date_of_birth": "1990-01-01",
                    "address": "123 Main St, Toronto, ON, M5V 3L9",
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
                    "address": "123 Main St, Toronto, ON, M5V 3L9",
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
            let consumer_credit_id = format!("{}_{}", test_uuid_matches, i + 1);
            let response = create_consumer_credit(
                &client,
                &consumer_credit_id,
                API_KEY_2.to_string(),
                &payload,
            );
            assert_eq!(response.status(), Status::Ok);
        }

        let response = view_consumer_match(&client, &*TEST_UUID, API_KEY_1.to_string());

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    #[serial]
    fn test_duplicate_consumer_credit_insertion() {
        let test_uuid_duplicate = Uuid::new_v4().to_string();
        let client = test_client().lock().unwrap();

        let dummy_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "date_of_birth": "1990-01-01",
                "address": "123 Main St, Toronto, ON, M5V 3L9",
                "phone_number": "+11234567890",
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
        let response = create_consumer_credit(
            &client,
            &test_uuid_duplicate,
            API_KEY_1.to_string(),
            &dummy_payload,
        );
        assert_eq!(response.status(), Status::Ok);

        // Second insertion with the same UUID should fail
        let response = create_consumer_credit(
            &client,
            &test_uuid_duplicate,
            API_KEY_1.to_string(),
            &dummy_payload,
        );
        assert_eq!(response.status(), Status::Conflict);
    }

    #[test]
    #[serial]
    fn test_update_consumer_credit() {
        let client = test_client().lock().unwrap();

        let dummy_payload = json!({
            "consumer_facts": {
                "last_name": "Randy",
            },
        });

        let response =
            update_consumer_credit(&client, &*TEST_UUID, API_KEY_1.to_string(), &dummy_payload);

        assert_eq!(response.status(), Status::Ok);

        let value = response_json_value(response);

        let last_name = value
            .get("consumer_facts")
            .expect("must have an 'consumer_facts' field")
            .get("last_name")
            .expect("must have a 'last_name' field")
            .as_str();

        assert_eq!(last_name, Some("Randy"));
    }
}
