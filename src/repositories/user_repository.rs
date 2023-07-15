use async_trait::async_trait;

use super::RepositoryResult;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: String,
    pub username: String,
    pub pw_hash: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_user_by_id(&self, id: &str) -> RepositoryResult<Option<UserEntity>>;
    async fn get_user_by_username(&self, username: &str) -> RepositoryResult<Option<UserEntity>>;
    async fn create_user(&self, username: &str, pw_hash: &str) -> RepositoryResult<String>;
}
