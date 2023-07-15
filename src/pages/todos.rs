use actix_web::{
    get, post,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use askama::Template;
use serde::Deserialize;

use crate::{
    repositories::{todo_repository::TodoEntity, user_repository::UserEntity},
    AppState, TemplateToResponse,
};

#[derive(Template, Default)]
#[template(path = "todos.html")]
struct TodosTemplate<'a> {
    todos: Vec<TodoEntity>,
    username: &'a str,
}

pub fn show_todos_page(username: &str, todos: Vec<TodoEntity>) -> HttpResponse {
    TodosTemplate { todos, username }.to_response()
}

#[get("/todos")]
async fn todos_page(state: web::Data<AppState>, req_user: ReqData<UserEntity>) -> impl Responder {
    let user_todos = state.todo_service.list_todos(&req_user.id).await.unwrap();
    show_todos_page(&req_user.username, user_todos)
}

#[derive(Deserialize, Debug)]
pub struct CreateTodoFormData {
    name: String,
}

#[post("/todos")]
pub async fn create_todo_submit(
    web::Form(form): web::Form<CreateTodoFormData>,
    state: web::Data<AppState>,
    req_user: ReqData<UserEntity>,
) -> impl Responder {
    state
        .todo_service
        .add_todo(&req_user.id, &form.name)
        .await
        .unwrap();

    let todos = state.todo_service.list_todos(&req_user.id).await.unwrap();

    show_todos_page(&req_user.username, todos)
}
