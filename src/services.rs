use crate::{AppState, AuthUser, TokenClaims};
use actix_web::{cookie::Cookie, http::header::LOCATION, post, web, HttpResponse, Responder};
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize)]
struct UserNoPassword {
    id: i32,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterFormData {
    username: String,
    password: String,
}

#[post("/register")]
pub async fn register_submit(
    web::Form(form): web::Form<RegisterFormData>,
    state: web::Data<AppState>,
) -> impl Responder {
    dbg!(&form);
    dbg!(&state);
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(form.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();
    let mut users = state.users.lock().unwrap();
    let id = users.len() as i32;
    users.push(AuthUser {
        id,
        username: form.username,
        password: hash,
    });

    HttpResponse::Found()
        .append_header((LOCATION, "/login"))
        .body("")
}

#[derive(Deserialize, Debug)]
pub struct LoginFormData {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login_submit(
    web::Form(form): web::Form<LoginFormData>,
    state: web::Data<AppState>,
) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();

    let db = state.users.lock().unwrap();
    match db.iter().find(|user| user.username == form.username) {
        Some(user) => {
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
            let mut verifier = Verifier::default();
            let is_valid = verifier
                .with_hash(user.password.clone())
                .with_password(form.password)
                .with_secret_key(hash_secret)
                .verify()
                .unwrap();

            if is_valid {
                let claims = TokenClaims { id: user.id };
                let access_token = claims.sign_with_key(&jwt_secret).unwrap();
                HttpResponse::Found()
                    .append_header((LOCATION, format!("/home/profile")))
                    .cookie(
                        Cookie::build("tokey", access_token)
                            .http_only(true)
                            .finish(),
                    )
                    .body("")
            } else {
                HttpResponse::Unauthorized().json("Incorrect username or password")
            }
        }
        None => HttpResponse::InternalServerError().body("user not found"),
    }
}
