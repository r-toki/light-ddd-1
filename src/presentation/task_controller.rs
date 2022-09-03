use super::result::Result;
use crate::application::task_service::CreateTaskInput;
use crate::context::Context;
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    Responder,
};
use serde::Deserialize;

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
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

#[derive(Deserialize)]
struct CreateTaskBody {
    description: String,
}
