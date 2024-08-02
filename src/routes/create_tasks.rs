use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use crate::database::{tasks, users::Model};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask{
    priority:Option<String>,
    title:String,
    description:Option<String>
}
pub async fn create_tasks(
    Extension(database):Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Json(request_task):Json<RequestTask>,
)-> Result<(), StatusCode>{
        let task = tasks::ActiveModel{
        priority:Set(request_task.priority), // Some is only required for optional variables
        title:Set(request_task.title),
        description:Set(request_task.description),
        user_id:Set(Some(user.id)),
        ..Default::default() //for auto increment id
    };
    let result = task.save(&database).await.unwrap();
    dbg!(result);
    Ok(())
}

// fn remove_bearer_prefix(token: &str) -> &str {
//     token.trim_start_matches("Bearer ").trim()
// }