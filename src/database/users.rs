use sqlx::Row;
use sqlx::{PgPool, postgres::PgRow};
use uuid::Uuid;

use crate::models::{User, error::UserError};

pub struct UserRepository;

impl UserRepository {
    /// Insere um novo usuário no banco de dados
    /// Retorna o ID do usuário criado ou erro se email já existir
    pub async fn insert(pool: &PgPool, user: User) -> Result<Uuid, UserError> {
        let email_exists: PgRow =
            sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(&user.email)
                .fetch_one(pool)
                .await?;

        let email_exists: bool = email_exists.get(0);

        if email_exists {
            return Err(UserError::EmailAlreadyExists);
        }

        let _ = sqlx::query(
            "INSERT INTO users (id, email, name, password_hash, is_active, created_at, updated_at)\
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.password_hash)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(pool)
        .await?;

        Ok(user.id)
    }

    /// Busca usuário por ID
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let maybe_row = sqlx::query("SELECT id, name, password_hash,is_active,created_at,updated_at FROM users WHERE id  = $1 AND is_active = true")
            .bind(user_id).fetch_optional(pool).await?;

        match maybe_row {
            Some(row) => Ok(Some(User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                is_active: row.get("is_active"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    /// Busca usuário por email (útil para login)
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
        let maybe_row =
            sqlx::query("SELECT id, name, email, password_hash FROM users WHERE email = $1")
                .bind(email.to_lowercase())
                .fetch_optional(pool)
                .await?;

        match maybe_row {
            Some(row) => {
                let user = User {
                    id: row.get("id"),
                    email: row.get("email"),
                    name: row.get("name"),
                    created_at: row.get("index"),
                    is_active: row.get("is_active"),
                    password_hash: row.get("password_hash"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    /// Lista usuários com paginação
    pub async fn find_all(pool: &PgPool, limit: i8, offset: i64) -> Result<Vec<User>, sqlx::Error> {
        let limit = if limit <= 0 { 1 } else { limit };
        let offset = if offset <= 0 { 1 } else { offset };
        let rows = sqlx::query(
            "SELECT id, name, email, password_hash FROM users ORDER BY name LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let mut users = Vec::new();

        for row in rows {
            let user = User {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                is_active: row.get("is_active"),
            };
            users.push(user);
        }

        Ok(users)
    }

    /// Atualiza dados do usuário
    pub async fn update(pool: &PgPool, user: User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE users
                   SET name = $1, email = $2, password_hash = $3, updated_at = $4
                   WHERE id = $5",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.password_hash)
        .bind(&user.updated_at)
        .bind(user.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Soft delete (marca como inativo)
    pub async fn delete(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE users SET is_active = false, updated_at = NOW() WHERE id = $1")
            .bind(&user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use serde::de::IntoDeserializer;

    use crate::models::{CreateUser, User};

    #[test]
    fn test_create_user() {
        let create_user = CreateUser {
            email: "fodase@gmail".into(),
            name: "fulano ciclano".into(),
            password: "Senha123456".into(),
        };

        let user = User::try_from(create_user);
    }
}
