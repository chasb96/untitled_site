use std::{error::Error, fmt::Display};
use jwt::Claims;
use serde_json::Value;

pub struct ClaimsUser {
    pub id: i32,
    pub username: String,
}

#[derive(Debug)]
pub enum UserClaimError {
    ClaimMissing(String),
    InvalidFormat(String),
}

impl Error for UserClaimError { }

impl Display for UserClaimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserClaimError::ClaimMissing(claim) => write!(f, "Missing {} claim", claim),
            UserClaimError::InvalidFormat(claim) => write!(f, "Invalid claim {}", claim),
        }
    }
}

impl TryFrom<Claims> for ClaimsUser {
    type Error = UserClaimError;

    fn try_from(claims: Claims) -> Result<Self, Self::Error> {
        let mut user_id = None;
        let mut uname = None;

        for (key, value) in claims.private {
            match key.as_str() {
                "user_id" => {
                    if !value.is_number() {
                        return Err(UserClaimError::InvalidFormat("user_id".to_string()))
                    }

                    user_id = Some(
                        value
                            .as_i64()
                            .unwrap() as i32
                    );
                },
                "username" => {
                    if !value.is_string() {
                        return Err(UserClaimError::InvalidFormat("username".to_string()))
                    }

                    uname = Some(
                        value
                            .as_str()
                            .unwrap()
                            .to_string()
                    );
                },
                _ => {},
            }
        }

        if user_id.is_none() { return  Err(UserClaimError::ClaimMissing("user_id".to_string())) }
        if uname.is_none() { return  Err(UserClaimError::ClaimMissing("username".to_string())) }

        Ok(
            ClaimsUser {
                id: user_id.unwrap(),
                username: uname.unwrap(),
            }
        )
    }
}

impl Into<Claims> for ClaimsUser {
    fn into(self) -> Claims {
        let mut claims = Claims::default();

        claims.private.insert(
            "user_id".to_string(), 
            Value::Number((self.id as i64).into())
        );

        claims.private.insert(
            "username".to_string(), 
            Value::String(self.username)
        );

        claims
    }
}