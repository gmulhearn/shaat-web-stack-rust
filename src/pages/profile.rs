use actix_web::{
    get,
    http::header::LOCATION,
    web::{Data, ReqData},
    HttpResponse, Responder,
};
use askama::Template;

use crate::{AppState, TemplateToResponse, TokenClaims};

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
}

#[get("/profile")]
async fn profile_page(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
) -> impl Responder {
    let user_id = if let Some(data) = req_user {
        data.id.clone()
    } else {
        return HttpResponse::Unauthorized().json("Unable to verify identity");
    };

    let user = state
        .user_repository
        .get_user_by_id(&user_id)
        .await
        .unwrap();

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
