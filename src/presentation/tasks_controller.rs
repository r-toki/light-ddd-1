use super::result::Result;
use crate::application::task_service::{
    CreateTaskInput, DeleteTaskInput, DoCompleteTaskInput, UndoCompleteTaskInput,
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

#[get("/tasks")]
async fn index(context: Data<Context>) -> Result<impl Responder> {
    let tasks = context.task_service.get_all_tasks().await?;
    Ok(Json(tasks))
}

#[post("/tasks")]
async fn create(context: Data<Context>, body: Json<CreateTaskBody>) -> Result<impl Responder> {
    context
        .task_service
        .create_task(CreateTaskInput {
            description: body.description.clone(),
        })
        .await?;
    Ok(Json(()))
}

#[delete("/tasks/{task_id}")]
async fn delete(context: Data<Context>, path: Path<TaskPath>) -> Result<impl Responder> {
    context
        .task_service
        .delete_task(DeleteTaskInput {
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
            .task_service
            .do_complete_task(DoCompleteTaskInput {
                id: path.task_id.clone(),
            })
            .await?;
        Ok(Json(()))
    }

    #[delete("/tasks/{task_id}/complete")]
    async fn delete(context: Data<Context>, path: Path<TaskPath>) -> Result<impl Responder> {
        context
            .task_service
            .undo_complete_task(UndoCompleteTaskInput {
                id: path.task_id.clone(),
            })
            .await?;
        Ok(Json(()))
    }
}

#[derive(Deserialize)]
struct TaskPath {
    pub task_id: String,
}

#[derive(Deserialize)]
struct CreateTaskBody {
    pub description: String,
}
