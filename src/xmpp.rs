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
}