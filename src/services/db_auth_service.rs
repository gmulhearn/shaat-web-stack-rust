use std::sync::Arc;

use argonautica::{Hasher, Verifier};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

use crate::{repositories::user_repository::UserRepository, TokenClaims};

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
        // TODO - no unwrap!
        let user_exists = self
            .user_repository
            .get_user_by_username(username)
            .await
            .unwrap()
            .is_some();
        if user_exists {
            return Err(AuthServiceError::UserAlreadyExists);
        }

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut hasher = Hasher::default();
        let hash = hasher
            .with_password(password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        self.user_repository
            .create_user(username, &hash)
            .await
            .unwrap();

        Ok(())
    }

    async fn authenticate_user(&self, username: &str, password: &str) -> AuthServiceResult<String> {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
        .unwrap();

        let user = self
            .user_repository
            .get_user_by_username(username)
            .await
            .unwrap()
            .ok_or(AuthServiceError::UserDoesNotExists)?;

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(user.pw_hash)
            .with_password(password)
            .with_secret_key(hash_secret)
            .verify()
            .unwrap();

        if !is_valid {
            return Err(AuthServiceError::IncorrectPassword);
        }

        let claims = TokenClaims { id: user.id };
        let access_token = claims.sign_with_key(&jwt_secret).unwrap();

        Ok(access_token)
    }
}