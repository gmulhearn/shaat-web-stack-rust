use async_trait::async_trait;

use super::RepositoryResult;

#[derive(Debug, Clone)]
pub struct TodoEntity {
    pub id: String,
    pub name: String,
    pub is_complete: bool,
}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn add_todo(&self, user_id: &str, name: &str) -> RepositoryResult<String>;
    async fn list_todos(&self, user_id: &str) -> RepositoryResult<Vec<TodoEntity>>;
    async fn remove_todo(&self, user_id: &str, id: &str) -> RepositoryResult<()>;
}
