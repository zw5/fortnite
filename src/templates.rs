#![allow(dead_code)]
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderValue, HeaderMap};

pub const FORTNITE_TOKEN: &str = "basic ZWM2ODRiOGM2ODdmNDc5ZmFkZWEzY2IyYWQ4M2Y1YzY6ZTFmMzFjMjExZjI4NDEzMTg2MjYyZDM3YTEzZmM4NGQ=";

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AuthDetails {
    pub access_token: String,
    expires_in: i64,
    expires_at: String,
    token_type: String,
    refresh_token: String,
    refresh_expires_at: String,
    pub account_id: String,
    client_id: String,
    internal_client: bool,
    client_service: String,
    app: String,
    in_app_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
    id: String,
    display_name: String,
    name: String,
    email: String,
    failed_login_attempts: i32,
    last_login: String,
    number_of_display_name_changes: i32,
    age_group: String,
    headless: bool,
    country: String,
    last_name: String,
    preferred_language: String,
    can_update_display_name: bool,
    tfa_enabled: bool,
    email_verified: bool,
    minor_verified: bool,
    minor_expected: bool,
    minor_status: String,
}

pub fn get_exchange_code_form(code: String) -> HashMap<String, String> {
    let mut hash_map = HashMap::new();
    hash_map.insert("grant_type".into(), "authorization_code".into());
    hash_map.insert("code".into(), code);
    hash_map
}

pub fn generate_headers(k: &'static str, v: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(k, HeaderValue::from_str(v).unwrap());
    headers
}