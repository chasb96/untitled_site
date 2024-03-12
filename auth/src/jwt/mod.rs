mod hmac;
mod claims_user;

pub use claims_user::ClaimsUser;

use std::{fmt::{Display, Debug}, error::Error};
use jwt::{Header, Token, Claims, VerifyWithKey};
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

#[derive(Debug)]
pub enum ValidateJwtError<T> {
    HmacKey(InvalidKeyError),
    Verify(jwt::Error),
    Claims(T),
}

impl<T: Display + Debug> Error for ValidateJwtError<T> { }

impl<T: Display> Display for ValidateJwtError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidateJwtError::HmacKey(e) => Display::fmt(e, f),
            ValidateJwtError::Verify(e) => Display::fmt(e, f),
            ValidateJwtError::Claims(e) => e.fmt(f),
        }
    }
}

impl<T> From<InvalidKeyError> for ValidateJwtError<T> {
    fn from(value: InvalidKeyError) -> Self {
        Self::HmacKey(value)
    }
}

impl<T> From<jwt::Error> for ValidateJwtError<T> {
    fn from(value: jwt::Error) -> Self {
        Self::Verify(value)
    }
}

pub fn verify_jwt<'a, T>(jwt: String) -> Result<T, ValidateJwtError<T::Error>> 
where
    T: TryFrom<Claims>
{
    let key = get_hmac_hasher()?;

    let token: Token<Header, Claims, _> = 
        jwt.verify_with_key(&key)?;

    Ok(
        token
            .claims()
            .clone()
            .try_into()
            .map_err(|e| ValidateJwtError::Claims(e))?
    )
}