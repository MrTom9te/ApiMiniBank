use actix_web::{
    HttpResponse, Responder, post,
    web::{self, Data, Json, ServiceConfig},
};
use sqlx::PgPool;

use crate::{
    database::UserRepository,
    models::{CreateUser, User, api_response::ApiResponse},
};

#[post("/register")]
async fn create_user(pool: Data<PgPool>, Json(create_user): Json<CreateUser>) -> impl Responder {
    let user = match User::try_from(create_user) {
        Ok(u) => u,
        Err(err) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "Falha ao criar usuario",
                &err.to_string(),
            ));
        }
    };
    // o PgPool já é Arc internamente → pode clonar sem custo
    match UserRepository::insert(&pool, user).await {
        Ok(uuid) => {
            HttpResponse::Created().json(ApiResponse::sucess(uuid, "Usuario criado com sucesso"))
        }
        Err(err) => HttpResponse::Conflict().json(ApiResponse::<()>::error(
            "Erro ao criar usuario, verifique informaçoes e tente novamente",
            &err.to_string(),
        )),
    }
}

pub fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").service(create_user));
}
