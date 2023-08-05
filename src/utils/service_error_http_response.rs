use actix_web::{http::StatusCode, HttpResponse, HttpResponseBuilder};

pub fn http_service_error_response(description: Option<String>) -> HttpResponse {
    let mut err_res = HttpResponseBuilder::new(StatusCode::INTERNAL_SERVER_ERROR);
    if let Some(desc) = description {
        err_res.body(desc)
    } else {
        err_res.finish()
    }
}
