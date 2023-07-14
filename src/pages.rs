use actix_web::{
    get,
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

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[get("/register")]
async fn register_page() -> impl Responder {
    RegisterTemplate.to_response()
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[get("/login")]
async fn login_page() -> impl Responder {
    LoginTemplate.to_response()
}

#[get("/profile")]
async fn profile(state: Data<AppState>, req_user: Option<ReqData<TokenClaims>>) -> impl Responder {
    let user_id = if let Some(data) = req_user {
        data.id
    } else {
        return HttpResponse::Unauthorized().json("Unable to verify identity");
    };

    let users = state.users.lock().unwrap();

    let user = users.iter().find(|user| user.id == user_id).unwrap();

    HelloTemplate {
        name: &user.username,
    }
    .to_response()
}
