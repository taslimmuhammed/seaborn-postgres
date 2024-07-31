use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

use crate::database::users;

#[derive(Deserialize)]
pub struct RequestAccount{
    username: String,
    password: String 
}
#[derive(Serialize)]
pub struct ResponseAccount{
    id:i32,
    username: String,
    token: String
}
pub async fn create_account(
    Extension(database):Extension<DatabaseConnection>,
    Json(account):Json<RequestAccount>
)->Result<Json<ResponseAccount>, StatusCode>{
    let new_user = users::ActiveModel{
        username: Set(account.username),
        password: Set(account.password),
        token: Set(Some(String::from("9823dwnoceu89384dj3d093"))),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(
        Json(
            ResponseAccount { id: new_user.id.unwrap(), username: new_user.username.unwrap(), token: new_user.token.unwrap().unwrap() }
        )
    )
}