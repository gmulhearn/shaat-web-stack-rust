pub mod askama_to_actix_responder;
pub mod services;

use std::sync::{Mutex, RwLock};

use actix_files::Files;
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
pub use askama_to_actix_responder::*;

use actix_web::{dev::ServiceRequest, get, web, App, Error, HttpMessage, HttpServer, Responder};
use askama::Template;
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use services::{basic_auth, create_article, create_user};
use sha2::Sha256;

#[derive(Template)] // this will generate the code...
#[template(path = "hello.html")] // using the template in this path, relative
                                 // to the `templates` dir in the crate root
struct HelloTemplate<'a> {
    // the name of the struct can be anything
    name: &'a str, // the field name should match the variable name
                   // in your template
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    // format!("Hello {}!", &name)
    HelloTemplate { name: &name }.to_response()
}

#[derive(Serialize)]
struct AuthUser {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub published_by: i32,
    pub published_on: Option<NaiveDateTime>,
}

#[derive(Default)]
pub struct AppState {
    users: Mutex<Vec<AuthUser>>,
    articles: Mutex<Vec<Article>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = web::Data::new(AppState::default());

    HttpServer::new(move || {
        let bearer_middleware = HttpAuthentication::bearer(validator);
        App::new()
            .app_data(app_state.clone())
            .service(basic_auth)
            .service(create_user)
            .service(
                web::scope("")
                    .wrap(bearer_middleware)
                    .service(create_article),
            )
            .service(Files::new("/static", "./static"))
            .service(hello)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
