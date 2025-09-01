use actix_files as fs;
use actix_web::{
    middleware::Logger,
    web::{self, ServiceConfig},
};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/")
                .wrap(Logger::default())
                .service(fs::Files::new("", "templates").index_file("index.html")),
        );
    };

    Ok(config.into())
}
