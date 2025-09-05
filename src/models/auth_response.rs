use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub id: String,
    pub email: String,
}
