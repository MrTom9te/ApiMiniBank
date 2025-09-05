use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::models::{User, error::UserError};

pub struct UserRepository;

impl UserRepository {
    /// Insere um novo usuário no banco de dados
    /// Retorna o ID do usuário criado ou erro se email já existir
    pub async fn insert(pool: &PgPool, user: User) -> Result<Uuid, UserError> {
        // Verifica se email já existe
        let email_exists: bool =
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(&user.email)
                .fetch_one(pool)
                .await?;

        if email_exists {
            return Err(UserError::EmailAlreadyExists);
        }

        // Insere o usuário
        sqlx::query(
            "INSERT INTO users (id, email, name, password_hash, is_active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.password_hash)
        .bind(user.is_active)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .execute(pool)
        .await?;

        Ok(user.id)
    }

    /// Busca usuário por ID
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, name, password_hash, is_active, created_at, updated_at
             FROM users WHERE id = $1 AND is_active = true",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    /// Busca usuário por email (útil para login)
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, email, name, password_hash, is_active, created_at, updated_at
             FROM users WHERE email = $1",
        )
        .bind(email.to_lowercase())
        .fetch_optional(pool)
        .await
    }

    /// Lista usuários com paginação
    pub async fn find_all(
        pool: &PgPool,
        limit: i32,
        offset: i64,
    ) -> Result<Vec<User>, sqlx::Error> {
        let limit = if limit <= 0 { 10 } else { limit };
        let offset = if offset < 0 { 0 } else { offset };

        sqlx::query_as::<_, User>(
            "SELECT id, email, name, password_hash, is_active, created_at, updated_at
             FROM users
             WHERE is_active = true
             ORDER BY name
             LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }

    /// Atualiza dados do usuário
    pub async fn update(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users
             SET name = $1, email = $2, password_hash = $3, updated_at = $4
             WHERE id = $5",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.updated_at)
        .bind(&user.id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Soft delete (marca como inativo)
    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users
             SET is_active = false, updated_at = NOW()
             WHERE id = $1",
        )
        .bind(&user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Conta o total de usuários (para paginação)
    pub async fn count_active(pool: &PgPool) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE is_active = true")
            .fetch_one(pool)
            .await
    }

    /// Verifica se um usuário existe e está ativo
    pub async fn exists_and_active(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1 AND is_active = true)")
            .bind(user_id)
            .fetch_one(pool)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateUser;

    #[tokio::test]
    async fn test_create_user() {
        let create_user = CreateUser {
            email: "test@gmail.com".to_string(), // Email válido
            name: "Fulano Ciclano".to_string(),
            password: "Senha123".to_string(),
        };

        let user_result = User::try_from(create_user);
        assert!(user_result.is_ok());

        let user = user_result.unwrap();
        assert_eq!(user.email, "test@gmail.com");
        assert_eq!(user.name, "Fulano Ciclano");
        assert!(user.is_active);
    }

    #[test]
    fn test_create_user_invalid_email() {
        let create_user = CreateUser {
            email: "fodase@gmail".to_string(), // Email inválido
            name: "Fulano Ciclano".to_string(),
            password: "Senha123".to_string(),
        };

        let user_result = User::try_from(create_user);
        assert!(user_result.is_err());

        match user_result.unwrap_err() {
            UserError::InvalidEmail(_) => {} // Esperado
            _ => panic!("Deveria ser InvalidEmail"),
        }
    }

    #[test]
    fn test_create_user_weak_password() {
        let create_user = CreateUser {
            email: "test@gmail.com".to_string(),
            name: "Fulano Ciclano".to_string(),
            password: "123".to_string(), // Senha fraca
        };

        let user_result = User::try_from(create_user);
        assert!(user_result.is_err());

        match user_result.unwrap_err() {
            UserError::WeakPassword => {} // Esperado
            _ => panic!("Deveria ser WeakPassword"),
        }
    }
}
