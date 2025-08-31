use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Dados para operações financeiras
#[derive(Debug, Deserialize)]
pub struct CreateTransaction {
    pub amount: Decimal,
    pub description: String,
}

/// Dados para transferência
#[derive(Debug, Deserialize)]
pub struct CreateTransfer {
    pub to_account_number: String,
    pub amount: Decimal,
    pub description: String,
}

/// Tipos de transação
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    TransferDebit,
    TransferCredit,
}

/// Status da transação
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
}

/// Entidade Transaction
#[derive(Debug, Serialize)]
pub struct Transaction {
    pub id: Uuid,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub amount: Decimal,
    pub transaction_type: TransactionType,
    pub description: String,
    pub reference_id: Option<Uuid>, // Para linking de transferências
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum TransactionError {
        #[error("Valor deve ser maior que zero")]
        InvalidAmount,

        #[error("Saldo insuficiente")]
        InsufficientFunds,

        #[error("Conta de origem não encontrada")]
        SourceAccountNotFound,

        #[error("Conta de destino não encontrada")]
        DestinationAccountNotFound,

        #[error("Não é possível transferir para a mesma conta")]
        SameAccountTransfer,

        #[error("Erro no banco de dados: {0}")]
        DatabaseError(#[from] sqlx::Error),
    }
}
