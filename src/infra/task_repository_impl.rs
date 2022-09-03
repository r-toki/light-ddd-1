use crate::domain::task::{task::Task, task_repository::TaskRepository};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct TaskRepositoryImpl {
    pool: PgPool,
}

impl TaskRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for TaskRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"
SELECT *
FROM tasks
ORDER BY created_at
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tasks)
    }

    async fn insert(&self, task: Task) -> Result<()> {
        sqlx::query!(
            r#"
INSERT INTO tasks ( id, description, completed, created_at, updated_at )
VALUES ( $1, $2, $3, $4, $5 )
            "#,
            task.id,
            task.description,
            task.completed,
            task.created_at,
            task.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
