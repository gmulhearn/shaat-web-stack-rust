use crate::{AppState, Article, AuthUser, TokenClaims};
use actix_web::{
    cookie::Cookie,
    get,
    http::header::{AUTHORIZATION, LOCATION},
    post,
    web::{Data, Json, Redirect, ReqData},
    HttpResponse, HttpResponseBuilder, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct UserNoPassword {
    id: i32,
    username: String,
}

#[derive(Deserialize)]
struct CreateArticleBody {
    title: String,
    content: String,
}

#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();
    let mut users = state.users.lock().unwrap();
    let id = users.len() as i32;
    users.push(AuthUser {
        id,
        username: user.username,
        password: hash,
    });

    HttpResponse::Ok()
}

// #[get("/auth2")]
// async fn basic_auth2(state: Data<AppState>) -> impl Responder {
//     let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
//         std::env::var("JWT_SECRET")
//             .expect("JWT_SECRET must be set!")
//             .as_bytes(),
//     )
//     .unwrap();
//     let username = "user";
//     let password = Some("pass");

//     match password {
//         None => HttpResponse::Unauthorized().json("Must provide username and password"),
//         Some(pass) => {
//             let db = state.users.lock().unwrap();
//             match db.iter().find(|user| user.username == username) {
//                 Some(user) => {
//                     let hash_secret =
//                         std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
//                     let mut verifier = Verifier::default();
//                     let is_valid = verifier
//                         .with_hash(user.password.clone())
//                         .with_password(pass)
//                         .with_secret_key(hash_secret)
//                         .verify()
//                         .unwrap();

//                     if is_valid {
//                         let claims = TokenClaims { id: user.id };
//                         let access_token = claims.sign_with_key(&jwt_secret).unwrap();
//                         // Redirect::to("http://0.0.0.0/here").see_other()
                        // HttpResponse::Found()
                        //     .append_header((LOCATION, "/home/potato"))
                        //     // .append_header((AUTHORIZATION, format!("Bearer: {access_token}")))
                        //     .cookie(
                        //         Cookie::build("tokey", access_token)
                        //             .http_only(true)
                        //             .finish(),
                        //     )
                        //     .body("")
//                     } else {
//                         HttpResponse::Unauthorized().json("Incorrect username or password")
//                     }
//                 }
//                 None => HttpResponse::InternalServerError().body("user not found"),
//             }
//         }
//     }
// }

#[get("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => {
            let db = state.users.lock().unwrap();
            match db.iter().find(|user| user.username == username) {
                Some(user) => {
                    let hash_secret =
                        std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password.clone())
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();

                    if is_valid {
                        let claims = TokenClaims { id: user.id };
                        let access_token = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Found()
                            .append_header((LOCATION, "/home/potato"))
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
    }
}

#[post("/article")]
async fn create_article(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    body: Json<CreateArticleBody>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            let article: CreateArticleBody = body.into_inner();

            let mut articles = state.articles.lock().unwrap();
            let id = articles.len() as i32;

            articles.push(Article {
                id,
                title: article.title,
                content: article.content,
                published_by: user.id,
                published_on: None,
            });

            HttpResponse::Ok().json("")
        }
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
    }
}
