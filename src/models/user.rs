use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::error::UserError, utils::hash_password, validators::UserValidator};

/// Dados que chegam do endpoint de registro
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password: String, // senha em texto claro
}

///login request
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}
///Resposta de login
#[derive(Debug, Serialize)]
pub struct LoginUserResponse {
    pub token: String,
    pub user_id: Uuid,
    pub email: String,
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
        let validated = UserValidator::validade_user_data(
            &create_user.name,
            &create_user.email,
            &create_user.password,
        )?;

        let password_hash =
            hash_password(&validated.password).map_err(|_| UserError::WeakPassword)?;

        let now = Utc::now();

        Ok(User {
            id: Uuid::new_v4(),
            email: validated.email,
            name: validated.name,
            password_hash: password_hash,
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
