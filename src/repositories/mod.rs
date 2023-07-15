pub mod in_memory_user_repository;
pub mod user_repository;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("already exists")]
    ItemAlreadyExists,
    #[error("unknown: {info:?}")]
    UnknownError { info: Option<String> },
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
