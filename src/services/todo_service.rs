use thiserror::Error;

use crate::repositories::todo_repository::{TodoEntity, TodoRepository};

#[derive(Error, Debug)]
pub enum TodoServiceError {
    #[error("Unknown error has occurred: {info:?}")]
    Unknown { info: Option<String> },
}

pub type TodoServiceResult<T> = Result<T, TodoServiceError>;

pub struct TodoService {
    todo_repository: Box<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(todo_repository: Box<dyn TodoRepository>) -> Self {
        Self { todo_repository }
    }

    pub async fn add_todo(&self, user_id: &str, name: &str) -> TodoServiceResult<()> {
        self.todo_repository.add_todo(user_id, name).await.unwrap();
        Ok(())
    }

    pub async fn list_todos(&self, user_id: &str) -> TodoServiceResult<Vec<TodoEntity>> {
        let todos = self.todo_repository.list_todos(user_id).await.unwrap();
        Ok(todos)
    }
}
