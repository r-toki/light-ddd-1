use super::result::Result;
use crate::context::Context;
use actix_web::{
    get,
    web::{Data, Json, ServiceConfig},
    Responder,
};

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(index);
}

#[get("/tasks")]
async fn index(context: Data<Context>) -> Result<impl Responder> {
    let tasks = context.task_service.find_all().await?;
    Ok(Json(tasks))
}
