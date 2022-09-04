use super::result::Result;
use crate::application::task_service::{
    CreateTaskUseCaseInput, DeleteTaskUseCaseInput, DoCompleteTaskUseCaseInput,
    UndoCompleteTaskUseCaseInput,
};
use crate::context::Context;
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(delete);
    cfg.service(complete::create);
    cfg.service(complete::delete);
}

#[derive(Deserialize)]
struct TaskPath {
    pub task_id: String,
}

#[get("/tasks")]
async fn index(context: Data<Context>) -> Result<impl Responder> {
    let tasks = context.get_all_tasks_use_case.execute().await?;
    Ok(Json(tasks))
}

#[derive(Deserialize)]
struct CreateTaskBody {
    pub description: String,
}

#[post("/tasks")]
async fn create(context: Data<Context>, body: Json<CreateTaskBody>) -> Result<impl Responder> {
    context
        .create_task_use_case
        .execute(CreateTaskUseCaseInput {
            description: body.description.clone(),
        })
        .await?;
    Ok(Json(()))
}

#[delete("/tasks/{task_id}")]
async fn delete(context: Data<Context>, path: Path<TaskPath>) -> Result<impl Responder> {
    context
        .delete_task_use_case
        .execute(DeleteTaskUseCaseInput {
            id: path.task_id.clone(),
        })
        .await?;
    Ok(Json(()))
}

mod complete {
    use super::*;

    #[post("/tasks/{task_id}/complete")]
    async fn create(context: Data<Context>, path: Path<TaskPath>) -> Result<impl Responder> {
        context
            .do_complete_task_use_case
            .execute(DoCompleteTaskUseCaseInput {
                id: path.task_id.clone(),
            })
            .await?;
        Ok(Json(()))
    }

    #[delete("/tasks/{task_id}/complete")]
    async fn delete(context: Data<Context>, path: Path<TaskPath>) -> Result<impl Responder> {
        context
            .undo_complete_task_use_case
            .execute(UndoCompleteTaskUseCaseInput {
                id: path.task_id.clone(),
            })
            .await?;
        Ok(Json(()))
    }
}
