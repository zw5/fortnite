#![allow(dead_code)]
use crate::http;
use crate::auth;
use crate::xmpp;

pub struct Client {
    http_client: http::Http,
    auth_client: auth::Authorization,
    xmpp_client: xmpp::XmppClient,
    ready: bool,
}

impl Client {
    fn new() -> Self {
        Self {
            http_client: http::Http::new(),
            auth_client: auth::Authorization::new(),
            xmpp_client: xmpp::XmppClient::new(),
            ready: false,
        }
    }

    async fn authenticate(mut self, authorization_code: String) -> Result<(), reqwest::Error> {
        self.http_client.resolve_exchange_code(&authorization_code.as_str()).await?;
        self.ready = true;
        Ok(())
    }

    pub async fn verify_token(self) {

    }
}