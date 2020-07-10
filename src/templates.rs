#![allow(dead_code)]
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthDetails {
    access_token: String,
    expires_in: i64,
    expires_at: String,
    token_type: String,
    refresh_token: String,
    refresh_expires_at: String,
    account_id: String,
    client_id: String,
    internal_client: bool,
    client_service: String,
    app: String,
    in_app_id: String,
    device_id: String
}

pub fn get_exchange_code_form(code: String) -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    hash_map.insert("grant_type".into(), "authorization_code".into());
    hash_map.insert("code".into(), code);
    hash_map
}