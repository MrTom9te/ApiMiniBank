use chrono::Utc;
use serde::Serialize;

/// Padronização do tipo de resposta
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub sucess: bool,
    pub data: Option<T>,
    pub message: String,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn sucess(data: T, message: &str) -> Self {
        Self {
            sucess: true,
            data: Some(data),
            message: message.into(),
            error: None,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
    pub fn error(message: &str, error: &str) -> Self {
        Self {
            sucess: false,
            data: None,
            message: message.into(),
            error: Some(error.into()),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
