use crate::domain::task::{
    task::{NewTask, Task},
    task_repository::TaskRepository,
};
use anyhow::Result;

#[derive(Clone)]
pub struct GetAllTasksUseCase<R: TaskRepository> {
    task_repository: R,
}

impl<R: TaskRepository> GetAllTasksUseCase<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self) -> Result<Vec<Task>> {
        self.task_repository.find_all().await
    }
}

#[derive(Clone)]

pub struct CreateTaskUseCase<R: TaskRepository> {
    task_repository: R,
}

pub struct CreateTaskUseCaseInput {
    pub description: String,
}

impl<R: TaskRepository> CreateTaskUseCase<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, input: CreateTaskUseCaseInput) -> Result<()> {
        let task = Task::new(NewTask {
            description: input.description.clone(),
        })?;
        self.task_repository.insert(task).await
    }
}

#[derive(Clone)]

pub struct DeleteTaskUseCase<R: TaskRepository> {
    task_repository: R,
}

pub struct DeleteTaskUseCaseInput {
    pub id: String,
}

impl<R: TaskRepository> DeleteTaskUseCase<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, input: DeleteTaskUseCaseInput) -> Result<()> {
        self.task_repository.delete(&input.id).await
    }
}

#[derive(Clone)]

pub struct DoCompleteTaskUseCase<R: TaskRepository> {
    task_repository: R,
}

pub struct DoCompleteTaskUseCaseInput {
    pub id: String,
}

impl<R: TaskRepository> DoCompleteTaskUseCase<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, input: DoCompleteTaskUseCaseInput) -> Result<()> {
        let mut task = self.task_repository.find_one(&input.id).await?;
        task.do_complete();
        self.task_repository.update(task).await
    }
}

#[derive(Clone)]

pub struct UndoCompleteTaskUseCase<R: TaskRepository> {
    task_repository: R,
}

pub struct UndoCompleteTaskUseCaseInput {
    pub id: String,
}

impl<R: TaskRepository> UndoCompleteTaskUseCase<R> {
    pub fn new(task_repository: R) -> Self {
        Self { task_repository }
    }

    pub async fn execute(&self, input: UndoCompleteTaskUseCaseInput) -> Result<()> {
        let mut task = self.task_repository.find_one(&input.id).await?;
        task.undo_complete();
        self.task_repository.update(task).await
    }
}
