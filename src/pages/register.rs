use actix_web::{get, http::header::LOCATION, post, web, HttpResponse, Responder};
use askama::Template;
use serde::Deserialize;

use crate::{services::auth_service::AuthServiceError, AppState, TemplateToResponse};

#[derive(Template, Default)]
#[template(path = "register.html")]
struct RegisterTemplate {
    error: Option<String>,
}

pub fn show_register_page(error: Option<&str>) -> HttpResponse {
    RegisterTemplate {
        error: error.map(String::from),
    }
    .to_response()
}

#[get("/register")]
async fn register_page() -> impl Responder {
    show_register_page(None)
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
    let res = state
        .auth_service
        .register_user(&form.username, &form.password)
        .await;
    match res {
        Ok(()) => HttpResponse::Found()
            .append_header((LOCATION, "/login"))
            .body(""),
        Err(AuthServiceError::UserAlreadyExists) => show_register_page(Some("User already exists")),
        Err(_) => show_register_page(Some("Unknown error")),
    }
}
