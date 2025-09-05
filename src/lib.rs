use std::sync::OnceLock;

use crate::handlers::auth_routes;
use actix_web::web::{self, ServiceConfig};

mod database;
mod handlers;
pub mod middleware;
mod models;
mod utils;
pub mod validators;

pub static JWT_SECRET: OnceLock<String> = OnceLock::new();

pub fn app(cgf: &mut ServiceConfig) {
    cgf.service(web::scope("/api").service(web::scope("/v1").configure(auth_routes)));
}
