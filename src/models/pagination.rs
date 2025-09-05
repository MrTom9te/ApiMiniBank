use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub pages: u32,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub sucess: bool,
    pub data: Vec<T>,
    pub pagination: Pagination,
    pub message: String,
    pub timestamp: String,
}

impl<T> PaginationResponse<T> {
    pub fn new(data: Vec<T>, pagination: Pagination, message: &str) -> Self {
        Self {
            sucess: true,
            data,
            pagination,
            message: message.into(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}
