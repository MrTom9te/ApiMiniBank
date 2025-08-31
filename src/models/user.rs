use bcrypt::{DEFAULT_COST, hash};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::error::UserError;

/// Dados que chegam do endpoint de registro
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password: String, // senha em texto claro
}

/// Entidade User final - pronta para persistência
#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<CreateUser> for User {
    type Error = UserError;
    fn try_from(create_user: CreateUser) -> Result<Self, Self::Error> {
        // Validação do nome - deve ter pelo menos 2 palavras
        let name_parts: Vec<&str> = create_user.name.trim().split_whitespace().collect();
        if name_parts.len() < 2 {
            return Err(UserError::InvalidName);
        }

        // Validação básica de email
        if !create_user.email.contains('@') || create_user.email.len() < 5 {
            return Err(UserError::InvalidEmail(create_user.email));
        }

        // Validação da senha - min 8 chars, 1 maiúscula, 1 número
        if create_user.password.len() < 8 {
            return Err(UserError::WeakPassword);
        }

        let has_uppercase = create_user.password.chars().any(|c| c.is_uppercase());
        let has_number = create_user.password.chars().any(|c| c.is_numeric());

        if !has_uppercase || !has_number {
            return Err(UserError::WeakPassword);
        }

        // Hash da senha
        let password_hash =
            hash(create_user.password, DEFAULT_COST).map_err(|_| UserError::WeakPassword)?;

        let now = Utc::now();

        Ok(User {
            id: Uuid::new_v4(),
            email: create_user.email.to_lowercase(),
            name: create_user.name.trim().to_string(),
            password_hash,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }
}

pub mod error {
    #[derive(Debug, thiserror::Error)]
    pub enum UserError {
        #[error("Email inválido: {0}")]
        InvalidEmail(String),

        #[error("Senha deve ter pelo menos 8 caracteres, 1 maiúscula e 1 número")]
        WeakPassword,

        #[error("Nome deve ter pelo menos 2 palavras")]
        InvalidName,

        #[error("Email já existe no sistema")]
        EmailAlreadyExists,

        #[error("Usuário não encontrado")]
        NotFound,

        #[error("Credenciais inválidas")]
        InvalidCredentials,

        #[error("Erro no banco de dados: {0}")]
        DatabaseError(#[from] sqlx::Error),
    }
}
