pub mod middleware;
pub mod pages;
pub mod repositories;
pub mod services;
pub mod utils;

use std::{env, sync::Arc};

use actix_files::Files;

use middleware::jwt_session::JwtSession;
use repositories::{
    in_memory_todo_repository::InMemoryTodoRepository,
    in_memory_user_repository::InMemoryUserRepository, sql_todo_repository::SqlTodoRepository,
    sql_user_repository::SqlUserRepository, user_repository::UserRepository,
};
use services::{
    auth_service::AuthService, db_auth_service::DbAuthService, todo_service::TodoService,
};
use sqlx::{Pool, Postgres};
pub use utils::askama_to_actix_responder::*;

use actix_web::{web, App, HttpServer};
use pages::{
    index::index_redirect,
    login::{login_page, login_submit},
    register::{register_page, register_submit},
    todos::{create_todo_submit, todos_page},
};
use serde::{Deserialize, Serialize};

pub struct AppState {
    auth_service: Box<dyn AuthService>,
    user_repository: Arc<dyn UserRepository>,
    todo_service: TodoService,
}

impl AppState {
    pub fn new_with_pool(pool: Pool<Postgres>) -> Self {
        let user_repo = Arc::new(SqlUserRepository::new(pool.clone())) as Arc<dyn UserRepository>;
        let auth_service = DbAuthService::new(Arc::clone(&user_repo).into());
        let todo_repo = Box::new(SqlTodoRepository::new(pool));
        let todo_service = TodoService::new(todo_repo);
        Self {
            auth_service: Box::new(auth_service),
            user_repository: user_repo,
            todo_service,
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let user_repo = Arc::new(InMemoryUserRepository::new()) as Arc<dyn UserRepository>;
        let auth_service = DbAuthService::new(Arc::clone(&user_repo).into());
        let todo_repo = Box::new(InMemoryTodoRepository::new());
        let todo_service = TodoService::new(todo_repo);
        Self {
            auth_service: Box::new(auth_service),
            user_repository: user_repo,
            todo_service,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: String,
    // TODO - issued, expiry, etc
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pg_url = env::var("DATABASE_URL").expect("DATABASE_URL must be in env");
    let pool: Pool<Postgres> = sqlx::postgres::PgPool::connect(&pg_url).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app_state = web::Data::new(AppState::new_with_pool(pool));

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            // TODO - figure out how to correctly order conflicting services with and without auth middleware
            .service(Files::new("/static", "./static"))
            .service(index_redirect)
            .service(register_page)
            .service(register_submit)
            .service(login_page)
            .service(login_submit)
            .service(
                web::scope("/home")
                    .wrap(JwtSession)
                    .service(todos_page)
                    .service(create_todo_submit),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
