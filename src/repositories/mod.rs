pub mod in_memory_todo_repository;
pub mod in_memory_user_repository;
pub mod sql_todo_repository;
pub mod sql_user_repository;
pub mod sqlx_error_mapper;
pub mod todo_repository;
pub mod user_repository;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("already exists")]
    ItemAlreadyExists,
    #[error("item not found")]
    ItemNotFound,
    #[error("database connection error {info:?}")]
    DatabaseConnectionError { info: Option<String> },
    #[error("unknown: {info:?}")]
    UnknownError { info: Option<String> },
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
