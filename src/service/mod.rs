use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub async fn default_api() -> &'static str {
    "Welcome to flash-sale 's open api."
}

pub mod consumer;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    code: i64,
    message: String,
    data: Option<BTreeMap<String, String>>,
}

impl Default for JsonResponse {
    fn default() -> Self {
        Self {
            code: 200,
            message: "Ok".to_string(),
            data: None,
        }
    }
}

impl JsonResponse {
    pub fn new(code: Option<i64>, message: Option<String>, data: Option<BTreeMap<String, String>>) -> Self {
        Self {
            code: match code {
                None => { 200 }
                Some(val) => { val }
            },
            message: match message {
                None => { "Ok".to_string() }
                Some(val) => { val }
            },
            data,
        }
    }
}