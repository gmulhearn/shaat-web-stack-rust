use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::utils::random_id;

use super::{
    user_repository::{UserEntity, UserRepository},
    RepositoryResult,
};

pub struct InMemoryUserRepository {
    users_by_id: Mutex<HashMap<String, UserEntity>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users_by_id: Default::default(),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn get_user_by_id(&self, id: &str) -> RepositoryResult<Option<UserEntity>> {
        let users = self.users_by_id.lock().await;
        let user = users.get(id).cloned();
        Ok(user)
    }

    async fn get_user_by_username(&self, username: &str) -> RepositoryResult<Option<UserEntity>> {
        let users = self.users_by_id.lock().await;
        let user = users
            .iter()
            .find_map(|(_, user)| (user.username == username).then(|| user.clone()));
        Ok(user)
    }

    async fn create_user(&self, username: &str, pw_hash: &str) -> RepositoryResult<String> {
        let id = random_id();

        let entity = UserEntity {
            id: id.clone(),
            username: username.to_owned(),
            pw_hash: pw_hash.to_owned(),
        };

        let mut users = self.users_by_id.lock().await;
        users.insert(id.clone(), entity);

        Ok(id)
    }
}
