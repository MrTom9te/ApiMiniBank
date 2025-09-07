use actix_web::{HttpResponse, Responder, delete, web};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    database::UserRepository,
    middleware,
    models::{api_response::ApiResponse, claims::Claims, error::UserError},
};
/// marca o usuario como inativo
#[delete("/account")]
async fn soft_delete_user(
    pool: web::Data<PgPool>,
    web::Json(email): web::Json<String>,
    claims: Claims,
) -> impl Responder {
    let user = match UserRepository::find_by_email(&pool, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NoContent().json(ApiResponse::<()>::error(
                "usuario nao encontrado",
                &UserError::NotFound.to_string(),
            ));
        }
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error("database_error", &err.to_string()));
        }
    };

    if &claims.email != &user.email {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            "NOT OWNER",
            &UserError::InvalidCredentials.to_string(),
        ));
    }

    if let Ok(_) = UserRepository::delete(&pool, user.id).await {
        HttpResponse::Ok().json(ApiResponse::<Uuid>::sucess(
            user.id,
            "conta desativada com sucesso!",
        ))
    } else {
        HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "Erro ao desativar usuario",
            &UserError::DatabaseError(sqlx::Error::PoolClosed).to_string(),
        ))
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .wrap(middleware::Authentication)
            .service(soft_delete_user),
    );
}
