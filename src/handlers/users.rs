use actix_web::{
    HttpResponse, HttpServer, Responder, post,
    web::{self, Data, Json, ServiceConfig},
};
use sqlx::PgPool;

use crate::{
    database::{RefreshTokenRepository, UserRepository},
    models::{
        CreateUser, LoginUserRequest, LoginUserResponse, User, api_response::ApiResponse,
        error::UserError,
    },
    utils::{create_token, create_token_refresh, verify_password},
    validators::LoginValidator,
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
    match UserRepository::insert(&pool, &user).await {
        Ok(uuid) => {
            HttpResponse::Created().json(ApiResponse::sucess(uuid, "Usuario criado com sucesso"))
        }
        Err(err) => HttpResponse::Conflict().json(ApiResponse::<()>::error(
            "Erro ao criar usuario, verifique informaçoes e tente novamente",
            &err.to_string(),
        )),
    }
}

#[post("/login")]
pub async fn login_user(pool: Data<PgPool>, login: Json<LoginUserRequest>) -> impl Responder {
    let validated_login = match LoginValidator::validate_login_data(&login.email, &login.password) {
        Ok(data) => data,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "INVALID INPUT",
                "Dados de login inválidos",
            ));
        }
    };

    let user = match UserRepository::find_by_email(&pool, &validated_login.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                &UserError::InvalidCredentials.to_string(),
                "Email ou senha incorretos",
            ));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                &UserError::InvalidCredentials.to_string(),
                "Erro interno do servidor",
            ));
        }
    };

    if !verify_password(&validated_login.password, &user.password_hash) {
        return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
            &UserError::InvalidCredentials.to_string(),
            "Email ou senha incorretos",
        ));
    }

    let token = match create_token(&user) {
        Ok(token) => token,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                "FAILED CREATE TOKEN",
                "Erro interno do servidore",
            ));
        }
    };

    let (refresh_token, expires_at) = create_token_refresh();

    if let Err(_) = RefreshTokenRepository::insert(&pool, user.id, &refresh_token, expires_at).await
    {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            "Erro a criar token",
            "Error ao criar token refresh",
        ));
    }

    HttpResponse::Ok().json(ApiResponse::sucess(
        LoginUserResponse {
            refresh_token,
            token,
            user_id: user.id,
            email: user.email,
        },
        "login efetuado com sucesso",
    ))
}

pub fn auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").service(create_user).service(login_user));
}
