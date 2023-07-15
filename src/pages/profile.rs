use actix_web::{get, web::ReqData, Responder};
use askama::Template;

use crate::{repositories::user_repository::UserEntity, TemplateToResponse};

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate<'a> {
    username: &'a str,
}

#[get("/profile")]
// ReqData<UserEntity> always available due to middleware guard
async fn profile_page(req_user: ReqData<UserEntity>) -> impl Responder {
    ProfileTemplate {
        username: &req_user.username,
    }
    .to_response()
}
