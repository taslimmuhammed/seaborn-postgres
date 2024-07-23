use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::database::tasks;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask{
    priority:Option<String>,
    title:String,
    description:Option<String>
}
pub async fn create_tasks(
    Extension(database):Extension<DatabaseConnection>,
    Json(request_task):Json<RequestTask>){
    let task = tasks::ActiveModel{
        priority:Set(request_task.priority), // Some is only required for optional variables
        title:Set(request_task.title),
        description:Set(request_task.description),
        ..Default::default() //for auto increment id
    };
    let result = task.save(&database).await.unwrap();
    dbg!(result);
}