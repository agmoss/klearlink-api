#[cfg(test)]
mod tests {
    use crate::rocket;
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;
    use serde_json::json;

    #[test]
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
                "consumer_state": "Active",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": 1000.0,
                "credit_type": "Personal Loan",
                "application_datetime": "2024-01-01T12:00:00Z",
                "credit_state": "Approved"
            }
        });

        let response = client
            .put("/consumer-credit/1")
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Created);
    }

    #[test]
    fn test_update_consumer_credit() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let dummy_payload = json!({
            "consumer_facts": {
                "first_name": "John",
                "last_name": "Doe",
                "email": "john.doe@example.com",
                "date_of_birth": "1990-01-01",
                "address": "123 Test St, Test City",
                "phone_number": "123-456-7890",
                "consumer_state": "Active",
                "institution_names": ["Bank A", "Bank B"]
            },
            "credit_facts": {
                "amount": 1000.0,
                "credit_type": "Personal Loan",
                "application_datetime": "2024-01-01T12:00:00Z",
                "credit_state": "Fulfilled"
            }
        });

        let response = client
            .post("/consumer-credit/1")
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .body(dummy_payload.to_string())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_view_consumer_credit() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/consumer-credit/1")
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_view_consumer_match() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/consumer-credit/1/consumer-match")
            .header(Header::new("X-API-Key", "test_key"))
            .header(Header::new("X-Username", "test_user"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
