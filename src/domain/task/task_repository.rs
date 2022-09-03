use super::task::Task;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>>;

    async fn insert(&self, task: Task) -> Result<()>;
}
