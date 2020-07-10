#![allow(dead_code)]
use crate::templates;
use crate::templates::AuthDetails;
use crate::errors;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use serde_json::to_string;

pub struct Http {
    http_client: reqwest::Client,
    headers: HeaderMap,
    auth_details: Option<AuthDetails>
}

impl Http {
    pub fn new() -> Http {
        Http {
            http_client: Client::new(),
            headers: HeaderMap::new(),
            auth_details: None
        }
    }
    async fn get(&self, url: &str) -> std::result::Result<reqwest::Response, reqwest::Error>{
        return self.http_client.get(url)
            .headers(self.headers.clone())
            .send()
            .await
    }
    async fn post(&self, url: &str, body: HashMap<String, String>) -> std::result::Result<reqwest::Response, reqwest::Error> {
        self.http_client.post(url)
            .body(to_string(&body).unwrap())
            .headers(self.headers.clone())
            .send()
            .await
    }
    pub async fn resolve_exchange_code(mut self, code: &str) -> std::result::Result<AuthDetails, reqwest::Error> {
        let epicgames_url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let string_code = code.into();
        self.headers.insert("Authorization", HeaderValue::from_str(templates::FORTNITE_TOKEN).unwrap());
        let details: templates::AuthDetails = self.post(epicgames_url, templates::get_exchange_code_form(string_code)).await?.json().await?;
        let str_token = &details.access_token[..];
        self.headers.insert("Authorization", HeaderValue::from_str(str_token).unwrap());
        println!("{:?}", &details);
        Ok(details)
    }

    pub async fn oauth_verify(self) -> errors::RefreshTokenResult<()> {
        Ok(())
    }

    pub fn add_header(mut self, headers: HeaderMap) -> Result<(), errors::InternalError>{
        for (key, value) in headers.into_iter() {
            let key = key.unwrap();
            self.headers.insert(key, value);
        }
        Ok(())
    }
    pub fn remove_header(mut self, header: HeaderName) -> Result<(), errors::InternalError> {
        self.headers.remove(header);
        Ok(())
    }

    async fn get_self_by_id(self) -> Result<(), reqwest::Error> {
        Ok(())
    }
}

