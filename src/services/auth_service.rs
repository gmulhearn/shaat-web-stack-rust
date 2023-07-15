use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User does not exist")]
    UserDoesNotExists,
    #[error("Incorrect password")]
    IncorrectPassword,
    #[error("Unknown error has occurred: {info:?}")]
    Unknown { info: Option<String> },
}

pub type AuthServiceResult<T> = Result<T, AuthServiceError>;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register_user(&self, username: &str, password: &str) -> AuthServiceResult<()>;
    async fn authenticate_user(&self, username: &str, password: &str) -> AuthServiceResult<String>;
}
