use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{ prelude::DateTimeWithTimeZone, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use crate::database::tasks::{self, Entity as Tasks};
#[derive(Serialize, Deserialize)]
pub struct ResponseTask{
    id:i32,
    title:String,
    priority:Option<String>,
    description:Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>
}
#[derive( Deserialize)]
pub struct PriorityQuery{
    priority:Option<String>
}

pub async fn get_one_task(
    Path(task_id):Path<i32>,
     Extension(database):Extension<DatabaseConnection>
)-> Result<Json<ResponseTask>, StatusCode>{
    let task = Tasks::find_by_id(task_id).one(&database).await.unwrap();
    if let Some(task) = task{
         Ok(Json(ResponseTask{
            id:task.id,
            title:task.title,
            priority:task.priority,
            description:task.description,
            deleted_at:task.deleted_at
        }))
    }else{
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_all_task(
    Extension(database):Extension<DatabaseConnection>,
    Query(q):Query<PriorityQuery>
)->Result<Json<Vec<ResponseTask>>, StatusCode>{
    let mut priority_filter =  Condition::all();
    // creating a new filter
    if let Some(priority) = q.priority{ // if only query exists
        priority_filter = if priority.is_empty(){ //handling the condition where string=""
            priority_filter.add(tasks::Column::Priority.is_null())
        }else{
            priority_filter.add(tasks::Column::Priority.eq(priority))
        }
    }
    let tasks = Tasks::find()
        .filter(priority_filter)
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|task| ResponseTask{
            id:task.id,
            title:task.title,
            priority:task.priority,
            description:task.description,
            deleted_at:task.deleted_at
        })
        .collect();
    Ok(Json(tasks)) 
}