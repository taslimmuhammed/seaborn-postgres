mod create_tasks;
mod get_task;
mod atomic_update;
mod partial_update;
mod delete;
use delete::delete_task;
use create_tasks::create_tasks;
use get_task::{get_one_task, get_all_task};
use atomic_update::atomic_update;
use partial_update::partial_update;
use sea_orm::DatabaseConnection;
use axum::{
    http::Method, routing::{delete, get, patch, post, put}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/tasks", post(create_tasks))
        .route("/get_all_tasks", get(get_all_task))
        .route("/get_one_task/:id", get(get_one_task))
        .route("/atomic_update/:task_id", put(atomic_update))
        .route("/partial_update/:task_id",patch(partial_update))
        .route("/delete/:task_id", delete(delete_task))
        .layer(Extension(database))
        .layer(cors);
    app
}