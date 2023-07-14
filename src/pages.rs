use actix_web::{
    get,
    http::header::LOCATION,
    web::{self, Data, ReqData},
    HttpResponse, Responder,
};
use askama::Template;

use crate::{AppState, TemplateToResponse, TokenClaims};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/{name}")]
async fn hello_page(name: web::Path<String>) -> impl Responder {
    HelloTemplate { name: &name }.to_response()
}

#[derive(Template, Default)]
#[template(path = "register.html")]
struct RegisterTemplate {
    error: Option<String>,
}

pub async fn show_register_page(error: Option<&str>) -> HttpResponse {
    RegisterTemplate {
        error: error.map(String::from),
    }
    .to_response()
}

#[get("/register")]
async fn register_page() -> impl Responder {
    show_register_page(None).await
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
}

pub async fn show_login_page(error: Option<&str>) -> HttpResponse {
    LoginTemplate {
        error: error.map(String::from),
    }
    .to_response()
}

#[get("/login")]
async fn login_page() -> impl Responder {
    show_login_page(None).await
}

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
}

#[get("/profile")]
async fn profile(state: Data<AppState>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
    let user_id = if let Some(data) = req_user {
        data.id
    } else {
        return HttpResponse::Unauthorized().json("Unable to verify identity");
    };

    let users = state.users.lock().unwrap();

    let user = users.iter().find(|user| user.id == user_id);

    match user {
        Some(user) => ProfileTemplate {
            username: &user.username,
        }
        .to_response(),
        None => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/login"))
            .body(""),
    }
}
