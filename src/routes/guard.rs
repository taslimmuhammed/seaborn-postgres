use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{database::users, utils::jwt};

pub async fn guard(mut request:Request, next:Next)-> Result<Response, StatusCode>{
    let token = request.headers()
        .get("authorization")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .unwrap()
        .trim_start_matches("Bearer ")
        .trim()
        .to_owned();
    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = users::Entity::find()
        .filter(users::Column::Token.eq(token.clone()))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    jwt::is_valid(&token)?; //use verfification after checking database since that is always faster, good against attackers
    let Some(user)  = user else {return Err(StatusCode::UNAUTHORIZED)};

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)

}