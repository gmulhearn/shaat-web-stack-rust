use std::fmt;

use actix_web::{ResponseError, HttpResponse, body::BoxBody, HttpResponseBuilder, http::{StatusCode, header::HeaderValue}};
use askama::Error;


/// Newtype to let askama::Error implement actix_web::ResponseError.
struct ActixError(Error);

impl fmt::Debug for ActixError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Error as fmt::Debug>::fmt(&self.0, f)
    }
}

impl fmt::Display for ActixError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Error as fmt::Display>::fmt(&self.0, f)
    }
}

impl ResponseError for ActixError {}

pub trait TemplateToResponse {
    fn to_response(&self) -> HttpResponse<BoxBody>;
}

impl<T: askama::Template> TemplateToResponse for T {
    fn to_response(&self) -> HttpResponse<BoxBody> {
        match self.render() {
            Ok(buffer) => HttpResponseBuilder::new(StatusCode::OK)
                .content_type(HeaderValue::from_static(T::MIME_TYPE))
                .body(buffer),
            Err(err) => HttpResponse::from_error(ActixError(err)),
        }
    }
}