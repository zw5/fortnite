#![allow(dead_code)]
struct Http {
    client: reqwest::Client
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginPost<'a > {
    grant_type: &'a str,
    code: &'a str
}

impl Default for LoginPost<'_> {
    fn default() -> LoginPost<'static> {
        LoginPost {
            grant_type: "authorization_code",
            code: "code"
        }
    }
}

struct Client {
    parent: Http
}

impl Http {

    async fn resolve_authorization_code(&self, authorization_code: &str) -> std::result::Result<reqwest::Response, reqwest::Error> {
        let url = "https://account-public-service-prod.ol.epicgames.com/account/api/oauth/token";
        let post_form = LoginPost {
            code: authorization_code,
            ..LoginPost::default()
        };
        let client = &self.client;
        return client.post(url)
            .json(&post_form)
            .send()
            .await;

    }

    async fn run(&mut self, authorization_code: &str) -> Result<(), reqwest::Error> {
        self.client = reqwest::Client::new();
        let code = self.resolve_authorization_code(authorization_code).await?;
        eprintln!("{:?}", code);
        Ok(())

    }

}
