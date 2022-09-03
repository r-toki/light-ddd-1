use crate::domain::task::{task::Task, task_repository::TaskRepository};
use anyhow::Result;

#[derive(Clone)]
pub struct TaskService<R>
where
    R: TaskRepository,
{
    task_repository: R,
}

impl<R> TaskService<R>
where
    R: TaskRepository,
{
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>> {
        self.task_repository.find_all().await
    }
}
