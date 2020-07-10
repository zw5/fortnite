#![allow(dead_code)]
use crate::templates;
use reqwest::Client;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use serde_json::to_string;

struct Http {
    http_client: reqwest::Client,
    headers: HeaderMap
}

impl Http {
    fn new() -> Http{
        Http {
            http_client: Client::new(),
            headers: HeaderMap::new(),
        }
    }
    async fn get(self, url: &str) -> std::result::Result<reqwest::Response, reqwest::Error>{
        return self.http_client.get(url)
            .headers(self.headers)
            .send()
            .await
    }
    async fn post(self, url: &str, body: HashMap<String, String>) -> std::result::Result<reqwest::Response, reqwest::Error> {
        self.http_client.post(url)
            .body(to_string(&body).unwrap())
            .headers(self.headers)
            .send()
            .await
    }
    pub async fn resolve_exchange_code(self, code: &str) {
        let epicgames_url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let string_code = code.into();
        let details = self.post(epicgames_url, templates::get_exchange_code_form(string_code)).await;
        println!("{:?}", details);
        return ()
    }


}