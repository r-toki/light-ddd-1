use crate::domain::task::{
    task::{NewTask, Task},
    task_repository::TaskRepository,
};
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

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>> {
        self.task_repository.find_all().await
    }

    pub async fn create_task(&self, input: CreateTaskInput) -> Result<()> {
        let task = Task::new(NewTask {
            description: input.description.clone(),
        })?;
        self.task_repository.insert(task).await
    }

    pub async fn delete_task(&self, input: DeleteTaskInput) -> Result<()> {
        self.task_repository.delete(&input.id).await
    }
}

pub struct CreateTaskInput {
    pub description: String,
}

pub struct DeleteTaskInput {
    pub id: String,
}
