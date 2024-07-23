mod create_tasks;
mod get_task;
mod update_one;
use create_tasks::create_tasks;
use get_task::{get_one_task, get_all_task};
use update_one::update_one;
use sea_orm::DatabaseConnection;
use axum::{
    http::Method, routing::{get, post, put}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/tasks", post(create_tasks))
        .route("/get_all_tasks", get(get_all_task))
        .route("/get_one_task/:id", get(get_one_task))
        .route("/update_one/:task_id", put(update_one))
        .layer(Extension(database))
        .layer(cors);
    app
}