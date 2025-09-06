use actix_web::{HttpMessage, body::BoxBody, dev, error::ErrorUnauthorized, middleware::Next};

use crate::{models::error::UserError, utils::verify_token};

pub async fn authentication(
    req: dev::ServiceRequest,
    next: Next<BoxBody>,
) -> Result<dev::ServiceResponse, actix_web::Error> {
    let auth_header = req.headers().get("Authorization");

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];

                match verify_token(token) {
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data.claims);
                        return next.call(req).await;
                    }
                    Err(_) => return Err(ErrorUnauthorized(UserError::InvalidCredentials)),
                }
            }
        }
    }

    Err(ErrorUnauthorized(UserError::InvalidCredentials))
}
