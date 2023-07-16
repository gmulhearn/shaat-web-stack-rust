use async_trait::async_trait;
use sqlx::{FromRow, Pool, Postgres};

use crate::utils::random_id;

use super::{
    user_repository::{UserEntity, UserRepository},
    RepositoryResult,
};

pub struct SqlUserRepository {
    pool: Pool<Postgres>,
}

impl SqlUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    id: String,
    username: String,
    password_hash: String,
}

#[async_trait]
impl UserRepository for SqlUserRepository {
    async fn get_user_by_id(&self, id: &str) -> RepositoryResult<Option<UserEntity>> {
        let query = sqlx::query_as!(UserRow, "SELECT * FROM Users WHERE id=$1", id);

        let user = query.fetch_optional(&self.pool).await.unwrap();

        Ok(user.map(From::from))
    }

    async fn get_user_by_username(&self, username: &str) -> RepositoryResult<Option<UserEntity>> {
        let query = sqlx::query_as!(UserRow, "SELECT * FROM Users WHERE username=$1", username);

        let user = query.fetch_optional(&self.pool).await.unwrap();

        Ok(user.map(From::from))
    }

    async fn create_user(&self, username: &str, pw_hash: &str) -> RepositoryResult<String> {
        let id = random_id();

        let query = sqlx::query!(
            "INSERT INTO Users (id, username, password_hash) 
            VALUES ($1, $2, $3)",
            id,
            username,
            pw_hash
        );

        query.execute(&self.pool).await.unwrap();

        Ok(id)
    }
}

impl From<UserRow> for UserEntity {
    fn from(value: UserRow) -> Self {
        UserEntity {
            id: value.id,
            username: value.username,
            pw_hash: value.password_hash,
        }
    }
}
