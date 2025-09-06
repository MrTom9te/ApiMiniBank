use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{User, error::UserError};

pub struct UserRepository;

impl UserRepository {
    /// Insere um novo usuário no banco de dados
    /// Retorna o ID do usuário criado ou erro se email já existir
    pub async fn insert(pool: &PgPool, user: &User) -> Result<Uuid, UserError> {
        let query = r#"
              INSERT INTO users (id, email, name, password_hash, is_active, created_at, updated_at)
              VALUES ($1, $2, $3, $4, $5, $6, $7)
              ON CONFLICT (email) DO NOTHING
              RETURNING id
          "#;
        let result: Option<Uuid> = sqlx::query_scalar(query)
            .bind(&user.id)
            .bind(&user.email)
            .bind(&user.name)
            .bind(&user.password_hash)
            .bind(user.is_active)
            .bind(&user.created_at)
            .bind(&user.updated_at)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(id) => Ok(id),
            None => Err(UserError::EmailAlreadyExists),
        }
    }

    /// Busca usuário por ID
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let query = r#"
                    SELECT id, email, name, password_hash, is_active, created_at, updated_at
                    FROM users
                    WHERE id = $1 AND is_active = true
                "#;
        sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
    }

    /// Busca usuário por email (útil para login)
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let query = r#"
                    SELECT id, email, name, password_hash, is_active, created_at, updated_at
                    FROM users
                    WHERE LOWER(email) = LOWER($1)
                "#;

        sqlx::query_as::<_, User>(query)
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
        let query = r#"
                   SELECT id, email, name, password_hash, is_active, created_at, updated_at
                   FROM users
                   WHERE is_active = true
                   ORDER BY name
                   LIMIT $1 OFFSET $2
               "#;
        sqlx::query_as::<_, User>(query)
            .bind(limit.max(1)) // se <= 0 vira 1
            .bind(offset.max(0)) // se < 0 vira 0
            .fetch_all(pool)
            .await
    }

    /// Atualiza dados do usuário
    pub async fn update(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
        let query = r#"
                    UPDATE users
                    SET name = $1, email = $2, password_hash = $3, is_active = $4, updated_at = $5
                    WHERE id = $6
                "#;

        sqlx::query(query)
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
        let query = r#"
                    UPDATE users
                    SET is_active = false, updated_at = NOW()
                    WHERE id = $1
                "#;

        sqlx::query(query).bind(&user_id).execute(pool).await?;

        Ok(())
    }

    /// Conta o total de usuários (para paginação)
    pub async fn count_active(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let query = r#"SELECT COUNT(*) FROM users WHERE is_active = true"#;
        sqlx::query_scalar(query).fetch_one(pool).await
    }

    /// Verifica se um usuário existe e está ativo
    pub async fn exists_and_active(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let query = r#"
                   SELECT EXISTS(
                       SELECT 1 FROM users WHERE id = $1 AND is_active = true
                   )
               "#;

        sqlx::query_scalar(query)
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
