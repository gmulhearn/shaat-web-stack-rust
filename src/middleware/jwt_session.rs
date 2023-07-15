//! JWT Session middleware. Does the following:
//! 1. extract the auth cookie from request (auth cookie should be set by POST /login)
//! 2. verify the token
//! 3. find the user from the repository
//! 4. add the UserEntity into the request extensions
//! If steps 1-3 fail or are not found, then the response will redirect to /login

// NOTE: combination of actix rules from:
// * async calls: https://github.com/actix/examples/blob/344bcfce10647748444695d3aa302ba3bb241310/middleware/middleware/src/read_request_body.rs
// * http redirecting: https://github.com/actix/examples/blob/344bcfce10647748444695d3aa302ba3bb241310/middleware/middleware/src/redirect.rs

use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::LOCATION,
    web::Data,
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jwt::VerifyWithKey;

use crate::{
    utils::global_auth::{get_jwt_signing_key, JWT_AUTH_COOKIE_NAME},
    AppState, TokenClaims,
};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtSession;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S: 'static, B> Transform<S, ServiceRequest> for JwtSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtSessionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtSessionMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtSessionMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let token_cookie = match req.cookie(JWT_AUTH_COOKIE_NAME) {
                Some(token) => token,
                None => return redirect_to_login_middleware_response(req),
            };
            let jwt_token = token_cookie.value();

            let key = get_jwt_signing_key();
            let claims = match VerifyWithKey::<TokenClaims>::verify_with_key(jwt_token, &key) {
                Ok(claims) => claims,
                Err(_) => return redirect_to_login_middleware_response(req),
            };

            let app_state = req
                .app_data::<Data<AppState>>()
                .expect("Fatal: could not access app data");
            let user = app_state
                .user_repository
                .get_user_by_id(&claims.id)
                .await
                .unwrap();

            let user = match user {
                Some(val) => val,
                None => return redirect_to_login_middleware_response(req),
            };

            req.extensions_mut().insert(user);

            let res = service.call(req).await?.map_into_left_body();
            Ok(res)
        })
    }
}

fn redirect_to_login_middleware_response<B: 'static>(
    req: ServiceRequest,
) -> Result<ServiceResponse<EitherBody<B>>, actix_web::Error> {
    let response = HttpResponse::Found()
        .insert_header((LOCATION, "/login"))
        .finish()
        // constructed responses map to "right" body
        .map_into_right_body();

    let (http_req, _) = req.into_parts();
    return Ok(ServiceResponse::new(http_req, response));
}
