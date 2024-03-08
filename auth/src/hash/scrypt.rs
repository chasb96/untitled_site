use std::{error::Error, fmt::Display};
use scrypt::{password_hash::{SaltString, rand_core::OsRng, PasswordHasher, PasswordHash, PasswordVerifier, Error as ScryptError}, Scrypt};

#[derive(Debug)]
pub struct PasswordHashError(ScryptError);

impl Error for PasswordHashError { }

impl Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<ScryptError> for PasswordHashError {
    fn from(value: ScryptError) -> Self {
        Self(value)
    }
}

pub fn generate_password_hash<'a>(password: &'a str) -> Result<String, PasswordHashError> {
    Scrypt
        .hash_password(
            password.as_bytes(), 
            &SaltString::generate(&mut OsRng)
        )
        .map(|hash| hash.to_string())
        .map_err(Into::into)
}

#[allow(unused)]
pub fn verify_password<'a>(password: &'a str, hash: &'a str) -> Result<bool, PasswordHashError> {
    Scrypt
        .verify_password(
            password.as_bytes(), 
            &PasswordHash::new(hash)?
        )
        .and_then(|_| Ok(true))
        .or_else(|e| match e {
            ScryptError::Password => Ok(false),
            _ => Err(PasswordHashError::from(e)),
        })
}

#[cfg(test)]
mod test {
    use super::{generate_password_hash, verify_password};

    #[test]
    pub fn test_generate_password_hash() {
        _ = generate_password_hash("hunter42").unwrap();
    }

    #[test]
    pub fn test_round() {
        let hashed = generate_password_hash("hunter42").unwrap();

        let verify = verify_password("hunter42", &hashed).unwrap();

        assert!(verify)
    }
}