use super::task::Task;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>>;

    async fn find_one(&self, id: &str) -> Result<Task>;

    async fn insert(&self, task: Task) -> Result<()>;

    async fn update(&self, task: Task) -> Result<()>;

    async fn delete(&self, id: &str) -> Result<()>;
}
