use futures_util::future::{Ready, ready};
use std::{future::Future, pin::Pin, rc::Rc};

use actix_web::{
    HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    error::ErrorUnauthorized,
};

use crate::{models::error::UserError, utils::verify_token};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Error = actix_web::Error;
    type Response = ServiceResponse<B>;

    type Transform = AuthenticationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");

            if let Some(auth_value) = auth_header {
                if let Ok(auth_str) = auth_value.to_str() {
                    let token = &auth_str[7..];
                    match verify_token(token) {
                        Ok(token) => {
                            req.extensions_mut().insert(token.claims);
                            return service.call(req).await;
                        }
                        Err(_) => return Err(ErrorUnauthorized(UserError::InvalidCredentials)),
                    }
                }
            }

            Err(ErrorUnauthorized(UserError::InvalidCredentials))
        })
    }
}
