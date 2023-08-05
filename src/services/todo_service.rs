use thiserror::Error;

use crate::repositories::{
    todo_repository::{TodoEntity, TodoRepository},
    RepositoryError,
};

#[derive(Error, Debug)]
pub enum TodoServiceError {
    #[error("Duplicate item")]
    DuplicateItem,
    #[error("Item not found")]
    ItemNotFound,
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
        self.todo_repository.add_todo(user_id, name).await?;
        Ok(())
    }

    pub async fn list_todos(&self, user_id: &str) -> TodoServiceResult<Vec<TodoEntity>> {
        let todos = self.todo_repository.list_todos(user_id).await?;
        Ok(todos)
    }
}

impl From<RepositoryError> for TodoServiceError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::ItemAlreadyExists => TodoServiceError::DuplicateItem,
            RepositoryError::UnknownError { info } => TodoServiceError::Unknown { info },
            RepositoryError::ItemNotFound => TodoServiceError::ItemNotFound,
            RepositoryError::DatabaseConnectionError { info } => TodoServiceError::Unknown { info },
        }
    }
}
