use sqlx::Row;
use sqlx::{PgPool, postgres::PgRow};
use uuid::Uuid;

use crate::models::{User, error::UserError};

pub struct UserRepository;

impl UserRepository {
    /// Insere um novo usuário no banco de dados
    /// Retorna o ID do usuário criado ou erro se email já existir
    pub async fn insert(pool: PgPool, user: User) -> Result<Uuid, UserError> {
        let email_exists: PgRow =
            sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
                .bind(&user.email)
                .fetch_one(&pool)
                .await?;

        let email_exists: bool = email_exists.get(0);

        if email_exists {
            return Err(UserError::EmailAlreadyExists);
        }

        sqlx::query(
            "INSERT INTO users (id, email, name, password_hash, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        ).bind(user.id).bind(&user.email).bind(&user.name).bind(&user.password_hash).bind(user.is_active).bind(user.created_at).bind(user.updated_at).execute(&pool).await?;

        Ok(user.id)
    }
    /// Busca usuário por ID
    pub async fn find_by_id(pool: PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let maybe_row = sqlx::query("SELECT id, name, password_hash,is_active,created_at,updated_at FROM users WHERE id  = $1 AND is_active = true")
            .bind(user_id).fetch_optional(&pool).await?;

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
}
