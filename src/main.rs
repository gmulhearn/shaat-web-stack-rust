pub mod middleware;
pub mod pages;
pub mod repositories;
pub mod services;
pub mod utils;

use std::sync::Arc;

use actix_files::Files;

use middleware::jwt_session::JwtSession;
use repositories::{
    in_memory_user_repository::InMemoryUserRepository, user_repository::UserRepository,
};
use services::{auth_service::AuthService, db_auth_service::DbAuthService};
pub use utils::askama_to_actix_responder::*;

use actix_web::{web, App, HttpServer};
use pages::{
    index::index_redirect,
    login::{login_page, login_submit},
    profile::profile_page,
    register::{register_page, register_submit},
};
use serde::{Deserialize, Serialize};

pub struct AppState {
    auth_service: Box<dyn AuthService>,
    user_repository: Arc<dyn UserRepository>,
}

impl Default for AppState {
    fn default() -> Self {
        let user_repo = Arc::new(InMemoryUserRepository::new()) as Arc<dyn UserRepository>;
        let auth_service = DbAuthService::new(Arc::clone(&user_repo).into());
        Self {
            auth_service: Box::new(auth_service),
            user_repository: user_repo,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: String,
    // TODO - issued, expiry, etc
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // TODO - figure out how to correctly order conflicting services with and without auth middleware
            .service(Files::new("/static", "./static"))
            .service(index_redirect)
            .service(register_page)
            .service(register_submit)
            .service(login_page)
            .service(login_submit)
            .service(web::scope("/home").wrap(JwtSession).service(profile_page))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
