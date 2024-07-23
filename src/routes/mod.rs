mod create_tasks;
mod get_one_task;
use create_tasks::create_tasks;
use get_one_task::get_one_task;
use sea_orm::{Database, DatabaseConnection};
use axum::{
    http::Method, middleware, routing::{get, post}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/tasks", post(create_tasks))
        .route("/get_one_task/:id", get(get_one_task))
        .layer(Extension(database))
        .layer(cors);
    app
}