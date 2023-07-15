use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::utils::random_id;

use super::{
    todo_repository::{TodoEntity, TodoRepository},
    RepositoryResult,
};

pub struct InMemoryTodoRepository {
    todos_by_user: Mutex<HashMap<String, HashMap<String, TodoEntity>>>,
}

impl InMemoryTodoRepository {
    pub fn new() -> Self {
        Self {
            todos_by_user: Default::default(),
        }
    }
}

#[async_trait]
impl TodoRepository for InMemoryTodoRepository {
    async fn add_todo(&self, user_id: &str, name: &str) -> RepositoryResult<String> {
        let mut todos_by_user = self.todos_by_user.lock().await;

        let id = random_id();
        let new_todo = TodoEntity {
            id: id.clone(),
            name: name.to_owned(),
            is_complete: false,
        };

        if let Some(todos) = todos_by_user.get_mut(user_id) {
            todos.insert(id.clone(), new_todo);
        } else {
            let new_map = HashMap::from([(id.clone(), new_todo)]);
            todos_by_user.insert(user_id.to_owned(), new_map);
        }

        Ok(id)
    }

    async fn list_todos(&self, user_id: &str) -> RepositoryResult<Vec<TodoEntity>> {
        let todos_by_user = self.todos_by_user.lock().await;
        Ok(todos_by_user.get(user_id).map_or(vec![], |entries| {
            entries.into_iter().map(|i| i.1.clone()).collect()
        }))
    }

    async fn remove_todo(&self, user_id: &str, id: &str) -> RepositoryResult<()> {
        let mut todos_by_user = self.todos_by_user.lock().await;

        if let Some(todos) = todos_by_user.get_mut(user_id) {
            todos.remove(id);
        };

        Ok(())
    }
}
