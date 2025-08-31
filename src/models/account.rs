use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Dados para criação de conta bancária
#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    pub account_type: AccountType,
}

/// Tipos de conta permitidos
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Checking,
    Savings,
    Investment,
}

/// Entidade Account
#[derive(Debug, Serialize)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_number: String,
    pub account_type: AccountType,
    pub balance: Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum AccountError {
        #[error("Conta não encontrada")]
        NotFound,

        #[error("Conta não pertence ao usuário")]
        Unauthorized,

        #[error("Conta está inativa")]
        Inactive,

        #[error("Número de conta já existe")]
        DuplicateAccountNumber,

        #[error("Erro no banco de dados: {0}")]
        DatabaseError(#[from] sqlx::Error),
    }
}
