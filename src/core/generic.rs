#![allow(clippy::extra_unused_lifetimes)]

use rocket::response::Responder;

use serde::{Deserialize, Serialize};

pub type JsonString = String;

#[derive(Serialize, Deserialize, Clone, Debug, Responder)]
pub struct ErrorMessage {
    pub error: JsonString,
}

impl ErrorMessage {
    pub fn json(&self) -> JsonString {
        serde_json::to_string(self).unwrap()
    }
}

impl From<&str> for ErrorMessage {
    fn from(msg: &str) -> Self {
        Self {
            error: msg.to_owned(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(msg: String) -> Self {
        Self { error: msg }
    }
}
