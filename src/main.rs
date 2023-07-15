pub mod pages;
pub mod repositories;
pub mod services;
pub mod utils;

use std::sync::Arc;

use actix_files::Files;
use actix_web_httpauth::{
    extractors::{
        bearer::{self},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use repositories::{
    in_memory_user_repository::InMemoryUserRepository, user_repository::UserRepository,
};
use services::{auth_service::AuthService, db_auth_service::DbAuthService};
pub use utils::askama_to_actix_responder::*;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpMessage, HttpServer};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use pages::{
    index::index_redirect,
    login::{login_page, login_submit},
    profile::profile_page,
    register::{register_page, register_submit},
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

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

async fn cookie_session_middleware(
    req: ServiceRequest,
    _srv: web::Data<AppState>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // TODO - wtf does this config mean
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default()
        .scope("/home");

    let token_cookie = match req.cookie("tokey") {
        Some(cookie) => cookie,
        None => return Err((AuthenticationError::from(config).into(), req)),
    };

    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = token_cookie.value();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        let session_middleware = HttpAuthentication::with_fn(cookie_session_middleware);

        App::new()
            .app_data(app_state.clone())
            // TODO - figure out how to correctly order conflicting services with and without auth middleware
            .service(Files::new("/static", "./static"))
            .service(index_redirect)
            .service(register_page)
            .service(register_submit)
            .service(login_page)
            .service(login_submit)
            .service(
                web::scope("/home")
                    .wrap(session_middleware)
                    .service(profile_page),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
