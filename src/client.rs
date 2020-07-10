#![allow(dead_code)]
use crate::http;
use crate::auth;    

pub struct Client {
    http_client: http::Http,
    auth_client: auth::Authorization,
}

impl Client {
    fn new() -> Self {
        Self {
            http_client: http::Http::new(),
            auth_client: auth::Authorization::new(),
        }
    }

    async fn authenticate(mut self, authorization_code: String) -> Result<(), reqwest::Error> {
        let res = self.http_client.resolve_exchange_code(&authorization_code.as_str()).await?; // fix spagetti later
        let res = self.auth_client.set_details(res);
        match res {
            Ok(_) => Ok(()),
            Err(_) => panic!("Lol couldn't auth"), // fix later
        }
    }
}