#![allow(dead_code)]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqwestBody {
    body: HashMap<String, String>
}

impl ReqwestBody {
    fn new(body: HashMap<String, String>) -> ReqwestBody {
        ReqwestBody {
            body: body
        }
    }
}

pub fn authorization_code(code: &str) -> ReqwestBody{
    let mut hash_map = HashMap::new();
    hash_map.insert("grant_type".to_string(),
            "authorization_code".to_string()).unwrap();
    hash_map.insert(
        "code".to_string(), code.to_string()).unwrap();
    ReqwestBody {
        body: hash_map
    }
}