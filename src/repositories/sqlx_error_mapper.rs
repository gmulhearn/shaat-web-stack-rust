use super::RepositoryError;

impl From<sqlx::Error> for RepositoryError {
    fn from(value: sqlx::Error) -> Self {
        let info = Some(value.to_string());
        match value {
            sqlx::Error::Configuration(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::Database(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::Io(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::Tls(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::Protocol(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::RowNotFound => RepositoryError::ItemNotFound,
            sqlx::Error::ColumnIndexOutOfBounds { .. } => RepositoryError::ItemNotFound,
            sqlx::Error::AnyDriverError(_) => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::PoolTimedOut => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::PoolClosed => RepositoryError::DatabaseConnectionError { info },
            sqlx::Error::WorkerCrashed => RepositoryError::DatabaseConnectionError { info },
            _ => RepositoryError::UnknownError { info },
        }
    }
}
