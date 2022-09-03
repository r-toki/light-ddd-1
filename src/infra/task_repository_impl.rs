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

    async fn find_one(&self, id: &str) -> Result<Task> {
        let task = sqlx::query_as!(
            Task,
            r#"
SELECT *
FROM tasks
WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(task)
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

    async fn update(&self, task: Task) -> Result<()> {
        sqlx::query!(
            r#"
UPDATE tasks
SET description = $1, completed = $2, created_at = $3, updated_at = $4
WHERE id = $5
            "#,
            task.description,
            task.completed,
            task.created_at,
            task.updated_at,
            task.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!(
            r#"
DELETE FROM tasks
WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
