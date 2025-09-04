use crate::JWT_SECRET;
use crate::models::{User, claims::Claims};
use bcrypt::{BcryptResult, DEFAULT_COST, hash, verify};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, encode};

pub fn create_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let exp = now + chrono::Duration::hours(1);
    let claims = Claims::new(
        user.id.clone().into(),
        exp.timestamp() as usize,
        now.timestamp() as usize,
        user.email.clone(),
    );

    let secret = get_jwt_secret(); // Pega o secret da variável global

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

pub fn verify_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = get_jwt_secret(); // Pega o secret da variável global

    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
}

pub fn get_jwt_secret() -> &'static str {
    JWT_SECRET.get().expect("JWT_SECRET não foi inicializado")
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}
