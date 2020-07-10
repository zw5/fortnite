#![allow(dead_code)]
use std::fmt;

pub type Result<T> = std::result::Result<T, HttpError>;

#[derive(Debug, Clone)]
pub struct HttpError;

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Http exception when making request.")
    }
}

pub type ErrorResult<T> = std::result::Result<T, InternalError>;

#[derive(Debug, Clone)]
pub struct InternalError;

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Http exception when making request.")
    }
}