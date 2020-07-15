#![allow(dead_code)]
use crate::templates;
use crate::templates::{UserDetails, AuthDetails};
use crate::errors;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

pub struct Http {
    http_client: reqwest::Client,
    headers: HeaderMap,
    pub auth_details: Option<AuthDetails>,
    user_details: Option<UserDetails>,
    pub ready: bool
}

impl Http {
    pub fn new() -> Result<Http, reqwest::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
        headers.insert(reqwest::header::USER_AGENT, HeaderValue::from_str("++Fortnite+Release-12.50-CL-13193885 Windows/10.0.17134.1.768.64bit").unwrap());
        let builder = reqwest::Client::builder()
            .build()?;
        Ok(Http {
            http_client: builder,
            headers: headers,
            auth_details: None,
            user_details: None,
            ready: false,
        })
    }
    async fn get(&self, url: &str) -> std::result::Result<reqwest::Response, reqwest::Error>{
        let proto = self.http_client.get(url)
            .headers(self.headers.clone());
        let proto = if let Some(auth_details) = &self.auth_details {
            proto.bearer_auth(auth_details.access_token.clone())
        } else { proto };
        proto.send().await
    }
    async fn post(&self, url: &str, body: HashMap<String, String>) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let proto = self.http_client.post(url)
            .json(&body)
            .headers(self.headers.clone());
        let proto = if let Some(auth_details) = &self.auth_details {
            proto.bearer_auth(auth_details.access_token.clone())
        } else { proto };
        proto.send().await
    }
    async fn post_form(&self, url: String, params: HashMap<&str, &str>) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let res = self.http_client.post(&url)
            .form(&params)
            .header("Authorization", templates::FORTNITE_TOKEN)
            .send()
            .await;
        return res;
    }
    pub async fn resolve_exchange_code(&mut self, code: &str) -> std::result::Result<(), reqwest::Error> {
        let epicgames_url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let authorization_form = hashmap!{
            "grant_type" => "authorization_code",
            "code" => code,
        };
        let res: AuthDetails = self.post_form(epicgames_url.to_string(), authorization_form).await?.json().await?;
        self.auth_details = Some(res);
        let details = self.get_self_by_id().await?;
        println!("{:?}", details);
        return Ok(());
    }

    pub async fn oauth_verify(self) -> errors::RefreshTokenResult<()> {
        Ok(())
    }

    pub async fn get_self_by_id(&self) -> Result<templates::UserDetails, reqwest::Error> {
        let details = if let Some(auth_details) = self.auth_details.clone() {
            auth_details
        } else { panic!("Not authenticated.") };
        let account_id = details.account_id;
        let url = format!("https://account-public-service-prod.ol.epicgames.com/account/api/public/account/{}", account_id);
        let res: templates::UserDetails = self.get(&url).await?.json().await?;
        Ok(res)
    }
}

