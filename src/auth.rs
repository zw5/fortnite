#![allow(dead_code)]
use crate::http::Client;
use std::option::Option;
use errors;

struct Auth<'a> {
    ios_token: String,
    launcher_token: String,
    fortnite_token: String,
    client: Option<&'a Client<'a>>,
    bearer_token: String,
}

impl Default for Auth<'_> {
    fn default() -> Auth<'static> {
        Auth {
            ios_token: "MzQ0NmNkNzI2OTRjNGE0NDg1ZDgxYjc3YWRiYjIxNDE6OTIwOWQ0YTVlMjVhNDU3ZmI5YjA3NDg5ZDMxM2I0MWE=".to_string(),
            launcher_token: "MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=".to_string(),
            fortnite_token: "ZWM2ODRiOGM2ODdmNDc5ZmFkZWEzY2IyYWQ4M2Y1YzY6ZTFmMzFjMjExZjI4NDEzMTg2MjYyZDM3YTEzZmM4NGQ=".to_string(),
            bearer_token: "".to_string(),
            client: None,
        }
    }
}

pub struct InternalAuthorizationCodeAuth<'a>{
    auth: Auth<'a>,
    code: String,
    client: Option<&'a Client<'a>>,
}

impl InternalAuthorizationCodeAuth<'_> {
    fn new(&mut self, auth_code: &str) -> InternalAuthorizationCodeAuth {
        Self {
            auth: Auth::default(),
            code: auth_code.to_string(),
            client: None,
        }
    }

     pub fn setup_internal(mut self, client: &'static Client) {
        self.client = Some(client);
    }

    async fn authenticate(&mut self) -> std::result::Result<(), errors::HttpError>{
        if let Some(c) = self.client {
            self.client.expect("No client to authorize.");
          } else {
            return errors::HttpError
          }
        let result = self.client.expect("No client to authorize").http.resolve_authorization_code(&self.code).await;
        match result {
            Ok(result) => result,
            Err(e) => Err(errors::HttpError)
        }
    }
}