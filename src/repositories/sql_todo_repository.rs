use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::utils::random_id;

use super::{
    todo_repository::{TodoEntity, TodoRepository},
    RepositoryResult,
};

pub struct SqlTodoRepository {
    pool: Pool<Postgres>,
}

impl SqlTodoRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(Debug)]
struct TodoRow {
    pub id: String,
    #[allow(unused)]
    pub user_id: String,
    pub name: String,
    pub complete: bool,
}

#[async_trait]
impl TodoRepository for SqlTodoRepository {
    async fn add_todo(&self, user_id: &str, name: &str) -> RepositoryResult<String> {
        let id = random_id();

        let query = sqlx::query!(
            "INSERT INTO Todos (id, user_id, name, complete) 
            VALUES ($1, $2, $3, $4)",
            id,
            user_id,
            name,
            false
        );

        query.execute(&self.pool).await.unwrap();

        Ok(id)
    }

    async fn list_todos(&self, user_id: &str) -> RepositoryResult<Vec<TodoEntity>> {
        let query = sqlx::query_as!(TodoRow, "SELECT * FROM Todos WHERE user_id=$1", user_id);

        let users = query.fetch_all(&self.pool).await.unwrap();

        Ok(users.into_iter().map(From::from).collect())
    }

    async fn remove_todo(&self, _user_id: &str, _id: &str) -> RepositoryResult<()> {
        todo!()
    }
}

impl From<TodoRow> for TodoEntity {
    fn from(value: TodoRow) -> Self {
        TodoEntity {
            id: value.id,
            name: value.name,
            is_complete: value.complete,
        }
    }
}
