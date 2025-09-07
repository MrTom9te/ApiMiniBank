use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

impl Claims {
    pub fn new(sub: String, exp: usize, iat: usize, email: String) -> Self {
        Self {
            sub,
            exp,
            iat,
            email,
        }
    }
}

impl FromRequest for Claims {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        match req.extensions().get::<Claims>() {
            Some(claims) => ready(Ok(claims.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized("Ivalid Claims"))),
        }
    }
}
