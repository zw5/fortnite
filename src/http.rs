#![allow(dead_code)]
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderValue;

use reqwest::header::HeaderMap;
use crate::auth;
use crate::templates;
use crate::errors;

pub struct Http {
    http_client: reqwest::Client,
    headers: HeaderMap,
}

pub struct Client {
    pub http: Http,
    pub auth: auth::InternalAuthorizationCodeAuth,
}

impl Http {

    async fn resolve_authorization_code(self, authorization_code: &str) -> Result<reqwest::Response, errors::HttpError> {
        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let post_form = templates::authorization_code(authorization_code);
        match self.post(url, post_form).await {
            Ok(x) => Ok(x),
            Err(_e) => Err(errors::HttpError),
        }
    }

    pub async fn post(self, url: &str, body: templates::ReqwestBody) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let http_client = &self.http_client;

        return http_client.post(url)
            .json(&body)
            .headers(self.headers)
            .send()
            .await;


    }



    fn new() -> Http {
        let mut initial_headers = HeaderMap::new();
        initial_headers
            .append(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        Http {
            http_client: reqwest::Client::new(),
            headers: initial_headers,
        }
    }

}

impl Client {
    pub fn new(auth: auth::InternalAuthorizationCodeAuth) -> Client {
        Client {
            http: Http::new(),
            auth: auth,
        }
    }

    pub async fn start(&mut self, authorization: auth::InternalAuthorizationCodeAuth) {
        self.auth = authorization;

    }
}