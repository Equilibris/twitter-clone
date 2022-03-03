use chrono::{serde::ts_seconds, DateTime, Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Token {
    // Maybe add IP check
    pub sub: Uuid,
    #[serde(with = "ts_seconds")]
    pub exp: DateTime<Utc>,
}

impl Token {
    pub fn new(sub: Uuid) -> Self {
        Self {
            sub,
            exp: Utc::now() + Duration::days(3),
        }
    }

    pub fn renew(&self) -> Self {
        Self::new(self.sub)
    }

    pub fn should_renew(self: &Self) -> bool {
        Utc::now() - self.exp > Duration::days(1)
    }

    pub fn encode(self: Self) -> errors::Result<String> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    }
    pub fn decode(string: String) -> errors::Result<jsonwebtoken::TokenData<Token>> {
        decode(
            &string,
            &DecodingKey::from_secret(crate::env::jwt_secret::get()),
            &Validation::new(Algorithm::HS256),
        )
    }
}
