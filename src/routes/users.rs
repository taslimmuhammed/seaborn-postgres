use axum::{http::{HeaderMap, StatusCode}, Extension, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,IntoActiveModel, Set};
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

pub async fn login_user(
    Extension(database):Extension<DatabaseConnection>,
    Json(account):Json<RequestAccount>
)->Result<Json<ResponseAccount>, StatusCode>{
    let db_user = users::Entity::find()
        .filter(users::Column::Username.eq(account.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user{
        let new_token = "ijrnu394r83nr9328di239".to_owned();
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(ResponseAccount{
            id: saved_user.id.unwrap(),
            username: saved_user.username.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    }else{
        Err(StatusCode::NOT_FOUND)
    }
    

}

pub async fn logout(
    headers: HeaderMap,
    Extension(database):Extension<DatabaseConnection>
)-> Result<(), StatusCode>{
    let token = headers.get("authorization").unwrap().to_str().unwrap();
    let token =token.trim_start_matches("Bearer ").trim();
    let mut user = if let Some(user) = users::Entity::find()
    .   filter(users::Column::Token.eq(token))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?{
            user.into_active_model()
        }else{
            return  Err(StatusCode::UNAUTHORIZED);
        };
    user.token = Set(None);
    let _ = user.save(&database).
        await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}