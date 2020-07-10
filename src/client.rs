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
}