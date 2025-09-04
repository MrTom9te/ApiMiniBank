use actix_files as fs;
use actix_web::{
    middleware::Logger,
    web::{self, ServiceConfig},
};
use api_mini_bank::JWT_SECRET;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Obtém o secret (corrigindo o typo "SECREAT" -> "SECRET")
    let secret = secrets
        .get("JWT_SECRET")
        .unwrap_or_else(|| "SEGREDO_PADRAO".to_string());

    // Define uma única vez - se tentar definir novamente, vai dar erro
    JWT_SECRET.set(secret).expect("JWT_SECRET já foi definido");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(Logger::default())
                .app_data(web::Data::new(pool.clone()))
                .configure(app)
                .service(fs::Files::new("/", "templates").index_file("index.html")),
        );
    };

    Ok(config.into())
}
