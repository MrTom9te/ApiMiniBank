use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, encode};

use crate::models::{User, claims::Claims};

pub fn create_jwt(user: &User, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + chrono::Duration::hours(1);

    let claims = Claims::new(
        user.id.clone().into(),
        exp.timestamp() as usize,
        now.timestamp() as usize,
        user.email.clone(),
    );

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}
