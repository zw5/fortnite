#![allow(dead_code)]
use crate::templates::AuthDetails;
use crate::errors;

pub struct Authorization {
    details: Option<AuthDetails>,
}

impl Authorization {
    pub fn new() -> Authorization {
        Authorization {
            details: None
        }
    }
    fn set_details(&mut self, details: AuthDetails) -> Result<(), errors::InternalError> {
        self.details = Some(details);
        Ok(())
    }
}