mod hmac;
mod claims_user;

pub use claims_user::ClaimsUser;

use std::{fmt::{Display, Debug}, error::Error};
use jwt::{Header, Token, SignWithKey, Claims, AlgorithmType};
use hmac::{get_hmac_hasher, InvalidKeyError};

#[derive(Debug)]
pub enum GenerateJwtError {
    HmacKey(InvalidKeyError),
    Signing(jwt::Error),
}

impl Error for GenerateJwtError { }

impl Display for GenerateJwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateJwtError::HmacKey(e) => write!(f, "Invalid HMAC Key: {}", e),
            GenerateJwtError::Signing(e) => write!(f, "Unable to sign Jwt: {}", e),
        }
    }
}

impl From<InvalidKeyError> for GenerateJwtError {
    fn from(value: InvalidKeyError) -> Self {
        Self::HmacKey(value)
    }
}

impl From<jwt::Error> for GenerateJwtError {
    fn from(value: jwt::Error) -> Self {
        Self::Signing(value)
    }
}

pub fn generate_jwt(content: impl Into<Claims>) -> Result<String, GenerateJwtError> {
    let key = get_hmac_hasher()?;

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };

    Token::new(header, content.into())
        .sign_with_key(&key)
        .map(|ok| ok.as_str().to_string())
        .map_err(GenerateJwtError::from)
}

#[derive(Debug)]
pub enum ValidateJwtError {
    HmacKey(InvalidKeyError),
    Verify(jwt::Error),
}

impl Error for ValidateJwtError { }

impl Display for ValidateJwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidateJwtError::HmacKey(e) => Display::fmt(e, f),
            ValidateJwtError::Verify(e) => Display::fmt(e, f),
        }
    }
}

impl From<InvalidKeyError> for ValidateJwtError {
    fn from(value: InvalidKeyError) -> Self {
        Self::HmacKey(value)
    }
}

impl From<jwt::Error> for ValidateJwtError {
    fn from(value: jwt::Error) -> Self {
        Self::Verify(value)
    }
}