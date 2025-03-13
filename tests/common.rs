use dotenvy::dotenv;
use klearlink_api;
use once_cell::sync::OnceCell;
use rocket::{
    http::Header,
    local::blocking::{Client, LocalResponse},
};
use serde_json::Value;
use std::sync::{Mutex, OnceLock};

pub fn response_json_value<'a>(response: LocalResponse<'a>) -> Value {
    let body = response.into_string().unwrap();
    serde_json::from_str(&body).expect("can't parse value")
}

pub fn test_client() -> &'static Mutex<Client> {
    dotenv().ok();
    static INSTANCE: OnceLock<Mutex<Client>> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        let rocket = klearlink_api::create_rocket();
        Mutex::new(Client::tracked(rocket).expect("valid Rocket client"))
    })

    
}

pub fn create_consumer_credit<'a>(
    client: &'a Client,
    consumer_credit_id: &'a String,
    api_key: String,
    payload: &Value,
) -> LocalResponse<'a> {
    let response = client
        .put(format!("/consumer-credit/{}", consumer_credit_id))
        .header(Header::new("Authorization", format!("Apikey {}", api_key)))
        .body(payload.to_string())
        .dispatch();

    response
}

pub fn update_consumer_credit<'a>(
    client: &'a Client,
    consumer_credit_id: &'a String,
    api_key: String,
    payload: &Value,
) -> LocalResponse<'a> {
    let response = client
        .post(format!("/consumer-credit/{}", consumer_credit_id))
        .header(Header::new("Authorization", format!("Apikey {}", api_key)))
        .body(payload.to_string())
        .dispatch();

    response
}

pub fn view_consumer_match<'a>(
    client: &'a Client,
    consumer_credit_id: &'a String,
    api_key: String,
) -> LocalResponse<'a> {
    let response = client
        .get(format!(
            "/consumer-credit/{}/consumer-match",
            consumer_credit_id
        ))
        .header(Header::new("Authorization", format!("Apikey {}", api_key)))
        .dispatch();

    response
}
