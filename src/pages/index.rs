use actix_web::{get, http::header::LOCATION, HttpResponse, Responder};

#[get("/")]
async fn index_redirect() -> impl Responder {
    // TODO - condition redirect based on auth
    HttpResponse::Found()
        .append_header((LOCATION, "/login"))
        .body("")
}
