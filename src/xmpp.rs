#![allow(dead_code)]
use crate::templates::AuthDetails;
use crate::errors::RefreshTokenResult;

pub struct XmppClient {
    auth_details: Option<AuthDetails>,
}

impl XmppClient {
    pub fn new() -> XmppClient{
        XmppClient {
            auth_details: None
        }
    }
    pub fn refresh_auth_details(mut self, auth_details: AuthDetails) -> RefreshTokenResult<()> {
        self.auth_details = Some(auth_details);
        Ok(())
    }
    pub fn start(self) {
    }
    fn jid(self, mut user_id: String) -> String {
        user_id.push_str("@prod.ol.epicgames.com/V2:Fortnite:WIN::");
        user_id.push_str(&uuid::Uuid::new_v4().to_string());
        user_id
    }
    pub fn connect(self, user_id: String, auth_token: String) {
        connect(self.jid(user_id), auth_token);
    }
}