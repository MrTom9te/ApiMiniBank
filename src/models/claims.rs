use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
