#![allow(dead_code)]
use crate::http;
use crate::xmpp;

pub struct Client {
    http_client: http::Http,
    xmpp_client: xmpp::XmppClient,
    ready: bool,
}

impl Client {
    pub fn new() -> Self {
        Self {
            http_client: http::Http::new().unwrap(),
            xmpp_client: xmpp::XmppClient::new(),
            ready: false,
        }
    }

    pub async fn authenticate(&mut self, authorization_code: String) -> Result<(), reqwest::Error> {
        self.http_client.resolve_exchange_code(&authorization_code.as_str()).await?;
        self.ready = true;
        Ok(())
    }

    pub async fn verify_token(self) {

    }

    pub fn connect_xmpp(self) {
        let auth_details = self.http_client.auth_details.clone();
        let user_id = if let Some(auth_details) = auth_details {
            auth_details.account_id
        } else { panic!("Not logged in") };// will fix the mess later
        let auth_details = self.http_client.auth_details.clone();
        let bearer = if let Some(auth_details) = auth_details {
            auth_details.access_token
        } else { panic!("Not logged in") };// will also fix this mess later
        self.xmpp_client.connect(user_id, bearer)
    }
}