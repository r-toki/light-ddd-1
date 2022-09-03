use crate::application::task_service::TaskService;
use crate::infra::task_repository_impl::TaskRepositoryImpl;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Context {
    pub task_service: TaskService<TaskRepositoryImpl>,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        let task_repository = TaskRepositoryImpl::new(pool.clone());

        let task_service = TaskService::new(task_repository.clone());

        Self { task_service }
    }
}
