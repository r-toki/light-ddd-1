use crate::application::task_service::{
    CreateTaskUseCase, DeleteTaskUseCase, DoCompleteTaskUseCase, GetAllTasksUseCase,
    UndoCompleteTaskUseCase,
};
use crate::infra::task_repository_impl::TaskRepositoryImpl;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Context {
    pub get_all_tasks_use_case: GetAllTasksUseCase<TaskRepositoryImpl>,
    pub create_task_use_case: CreateTaskUseCase<TaskRepositoryImpl>,
    pub delete_task_use_case: DeleteTaskUseCase<TaskRepositoryImpl>,
    pub do_complete_task_use_case: DoCompleteTaskUseCase<TaskRepositoryImpl>,
    pub undo_complete_task_use_case: UndoCompleteTaskUseCase<TaskRepositoryImpl>,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        let task_repository = TaskRepositoryImpl::new(pool.clone());

        let get_all_tasks_use_case = GetAllTasksUseCase::new(task_repository.clone());
        let create_task_use_case = CreateTaskUseCase::new(task_repository.clone());
        let delete_task_use_case = DeleteTaskUseCase::new(task_repository.clone());
        let do_complete_task_use_case = DoCompleteTaskUseCase::new(task_repository.clone());
        let undo_complete_task_use_case = UndoCompleteTaskUseCase::new(task_repository.clone());

        Self {
            get_all_tasks_use_case,
            create_task_use_case,
            delete_task_use_case,
            do_complete_task_use_case,
            undo_complete_task_use_case,
        }
    }
}
