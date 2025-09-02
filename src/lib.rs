use actix_web::web::{self, ServiceConfig};

use crate::handlers::auth_routes;

mod database;
mod handlers;
mod models;

pub fn app(cgf: &mut ServiceConfig) {
    cgf.service(web::scope("/api").service(web::scope("/v1").configure(auth_routes)));
}
