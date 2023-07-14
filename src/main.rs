pub mod askama_to_actix_responder;
pub mod pages;
pub mod services;

use std::sync::Mutex;

use actix_files::Files;
use actix_web_httpauth::{
    extractors::{
        bearer::{self},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
pub use askama_to_actix_responder::*;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpMessage, HttpServer};
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use pages::{login_page, profile, register_page};
use serde::{Deserialize, Serialize};
use services::{login_submit, register_submit};
use sha2::Sha256;

#[derive(Serialize, Debug)]
struct AuthUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published_by: i32,
    pub published_on: Option<NaiveDateTime>,
}

#[derive(Default, Debug)]
pub struct AppState {
    users: Mutex<Vec<AuthUser>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
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
            .service(register_page)
            .service(register_submit)
            .service(login_page)
            .service(login_submit)
            .service(
                web::scope("/home")
                    .wrap(session_middleware)
                    // .service(create_article)
                    .service(profile), // .service(hello_page),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
