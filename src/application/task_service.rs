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

    pub async fn do_complete_task(&self, input: DoCompleteTaskInput) -> Result<()> {
        let mut task = self.task_repository.find_one(&input.id).await?;
        task.do_complete();
        self.task_repository.update(task).await
    }

    pub async fn undo_complete_task(&self, input: UndoCompleteTaskInput) -> Result<()> {
        let mut task = self.task_repository.find_one(&input.id).await?;
        task.undo_complete();
        self.task_repository.update(task).await
    }
}

pub struct CreateTaskInput {
    pub description: String,
}

pub struct DeleteTaskInput {
    pub id: String,
}

pub struct DoCompleteTaskInput {
    pub id: String,
}

pub struct UndoCompleteTaskInput {
    pub id: String,
}
