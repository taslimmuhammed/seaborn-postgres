use axum::{http::{header, HeaderMap, StatusCode}, Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use crate::database::{tasks, users};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestTask{
    priority:Option<String>,
    title:String,
    description:Option<String>
}
pub async fn create_tasks(
    headers:HeaderMap,
    Extension(database):Extension<DatabaseConnection>,
    Json(request_task):Json<RequestTask>,
)-> Result<(), StatusCode>{
    dbg!(headers.clone().get("authorization").unwrap().to_str().unwrap());
    let token = headers.get("authorization").unwrap().to_str().unwrap();
    let token = remove_bearer_prefix(token);
    let user = if let Some(user) = users::Entity::find()
        .filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            user
    }else{
        return Err(StatusCode::UNAUTHORIZED)
    };

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

fn remove_bearer_prefix(token: &str) -> &str {
    token.trim_start_matches("Bearer ").trim()
}