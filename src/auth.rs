use crate::http::Client;
use crate::http;
use std::option::Option;

struct Auth {
    ios_token: String,
    launcher_token: String,
    fortnite_token: String,
    client: Option<*mut Client>,
    bearer_token: String,
}

impl Default for Auth {
    fn default() -> Auth {
        Auth {
            ios_token: "MzQ0NmNkNzI2OTRjNGE0NDg1ZDgxYjc3YWRiYjIxNDE6OTIwOWQ0YTVlMjVhNDU3ZmI5YjA3NDg5ZDMxM2I0MWE=".to_string(),
            launcher_token: "MzRhMDJjZjhmNDQxNGUyOWIxNTkyMTg3NmRhMzZmOWE6ZGFhZmJjY2M3Mzc3NDUwMzlkZmZlNTNkOTRmYzc2Y2Y=".to_string(),
            fortnite_token: "ZWM2ODRiOGM2ODdmNDc5ZmFkZWEzY2IyYWQ4M2Y1YzY6ZTFmMzFjMjExZjI4NDEzMTg2MjYyZDM3YTEzZmM4NGQ=".to_string(),
            bearer_token: "".to_string(),
            client: None,
        }
    }
}

pub struct InternalAuthorizationCodeAuth{
    auth: Auth,
    code: String,
    client: Option<*mut Client>,
}

impl InternalAuthorizationCodeAuth {
    fn new(&mut self, auth_code: &str) -> InternalAuthorizationCodeAuth {
        Self {
            auth: Auth::default(),
            code: auth_code.to_string(),
            client: None,
        }
    }

     pub fn setup_internal(mut self, client: *mut Client) {
        self.client = Some(client);
    }

    fn authenticate(&mut self) {
        let _http = match &(*self.client).http {
            Some(http) => http,
            None => panic!("Internals weren't set up.")
        };



    }
}