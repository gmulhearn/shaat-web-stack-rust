use std::sync::Arc;

use argonautica::{Hasher, Verifier};
use async_trait::async_trait;
use chrono::Utc;
use jwt::SignWithKey;

use crate::{
    repositories::{user_repository::UserRepository, RepositoryError},
    utils::global_auth::{get_jwt_signing_key, get_password_hash_secret, JWT_AUTH_EXPIRATION_MINS},
    TokenClaims,
};

use super::auth_service::{AuthService, AuthServiceError, AuthServiceResult};

pub struct DbAuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl DbAuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        DbAuthService { user_repository }
    }
}

#[async_trait]
impl AuthService for DbAuthService {
    async fn register_user(&self, username: &str, password: &str) -> AuthServiceResult<()> {
        let user_exists = self
            .user_repository
            .get_user_by_username(username)
            .await?
            .is_some();
        if user_exists {
            return Err(AuthServiceError::UserAlreadyExists);
        }

        let hash_secret = get_password_hash_secret();
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key(hash_secret)
            .hash()
            .map_err_unknown()?;

        self.user_repository.create_user(username, &hash).await?;

        Ok(())
    }

    async fn authenticate_user(&self, username: &str, password: &str) -> AuthServiceResult<String> {
        let user = self
            .user_repository
            .get_user_by_username(username)
            .await?
            .ok_or(AuthServiceError::UserDoesNotExists)?;

        let hash_secret = get_password_hash_secret();
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(user.pw_hash)
            .with_password(password)
            .with_secret_key(hash_secret)
            .verify()
            .map_err_unknown()?;

        if !is_valid {
            return Err(AuthServiceError::IncorrectPassword);
        }

        let issued = Utc::now();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(JWT_AUTH_EXPIRATION_MINS))
            .expect("valid timestamp");

        let claims = TokenClaims {
            id: user.id,
            expiration,
            issued,
        };
        let jwt_secret = get_jwt_signing_key();
        let access_token = claims.sign_with_key(&jwt_secret).map_err_unknown()?;

        Ok(access_token)
    }
}

impl From<RepositoryError> for AuthServiceError {
    fn from(value: RepositoryError) -> Self {
        match value {
            RepositoryError::ItemAlreadyExists => AuthServiceError::UserAlreadyExists,
            RepositoryError::UnknownError { info } => AuthServiceError::Unknown { info },
            RepositoryError::ItemNotFound => AuthServiceError::UserDoesNotExists,
            RepositoryError::DatabaseConnectionError { info } => AuthServiceError::Unknown { info },
        }
    }
}

trait MapUnknown {
    type MappedRes;
    fn map_err_unknown(self) -> Self::MappedRes;
}

impl<T, E: std::fmt::Display> MapUnknown for Result<T, E> {
    type MappedRes = Result<T, AuthServiceError>;

    fn map_err_unknown(self) -> Self::MappedRes {
        self.map_err(|e| AuthServiceError::Unknown {
            info: Some(e.to_string()),
        })
    }
}
