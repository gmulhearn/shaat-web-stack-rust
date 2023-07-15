use actix_web::{cookie::Cookie, get, http::header::LOCATION, post, web, HttpResponse, Responder};
use askama::Template;
use serde::Deserialize;

use crate::{services::auth_service::AuthServiceError, AppState, TemplateToResponse};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    error: Option<String>,
}

pub fn show_login_page(error: Option<&str>) -> HttpResponse {
    LoginTemplate {
        error: error.map(String::from),
    }
    .to_response()
}

#[get("/login")]
async fn login_page() -> impl Responder {
    show_login_page(None)
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
    let res = state
        .auth_service
        .authenticate_user(&form.username, &form.password)
        .await;

    let access_token = match res {
        Ok(token) => token,
        Err(AuthServiceError::IncorrectPassword) => {
            return show_login_page(Some("Incorrect username or password"))
        }
        Err(AuthServiceError::UserDoesNotExists) => {
            return show_login_page(Some("Incorrect username or password"))
        }
        _ => return show_login_page(Some("Unknown error")),
    };

    HttpResponse::Found()
        .append_header((LOCATION, format!("/home/profile")))
        .cookie(
            Cookie::build("tokey", access_token)
                .http_only(true)
                .finish(),
        )
        .body("")
}
