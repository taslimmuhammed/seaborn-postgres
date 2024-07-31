use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};

use crate::database::tasks;

pub async fn delete_task(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
)->Result<(),StatusCode>{
    // let task = if let Some(task) = tasks::Entity::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
    //         task.into_active_model()
    //     }else{ 
    //         return Err(StatusCode::NOT_FOUND)
    //     };
    // tasks::Entity::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Ok(())
    tasks::Entity::delete_many()
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn soft_delete(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
)->Result<(),StatusCode>{
    let mut task = if let Some(task) = tasks::Entity::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            task.into_active_model()
        }else{
            return Err(StatusCode::NOT_FOUND);
        };
    let now = chrono::Utc::now();
    task.deleted_at = Set(Some(now.into()));
    tasks::Entity::update(task)
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}