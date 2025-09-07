use sqlx::PgPool;
use uuid::Uuid;

pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    pub async fn insert(
        pool: &PgPool,
        user_id: Uuid,
        refresh_token: &str,
        expires_at: i64,
    ) -> Result<(), sqlx::Error> {
        let query = r#"
            insert into refresh_tokens (user_id,token,expires_at)
            values ($1,$2,$3,$4)
            "#;
        sqlx::query(query)
            .bind(user_id)
            .bind(refresh_token)
            .bind(expires_at)
            .execute(pool)
            .await?;
        Ok(())
    }
}
