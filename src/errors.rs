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

pub type InternalResult<T> = std::result::Result<T, InternalError>;

#[derive(Debug, Clone)]
pub struct InternalError;

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal library error.")
    }
}

pub type AuthorizationResult<T> = std::result::Result<T, AuthorizationCodeError>;

#[derive(Debug, Clone)]
pub struct AuthorizationCodeError;

impl fmt::Display for AuthorizationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid exchange code")
    }
}

pub type RefreshTokenResult<T> = std::result::Result<T, RefreshTokenError>;

#[derive(Debug, Clone)]
pub struct RefreshTokenError;

impl fmt::Display for RefreshTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid exchange code")
    }
}