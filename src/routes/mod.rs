use sea_orm::{Database, DatabaseConnection};
mod create_tasks;
use create_tasks::create_tasks;
use axum::{
    http::Method, middleware, routing::{get, post}, Extension, Router
};
use tower_http::cors::{Any, CorsLayer};
pub fn create_routes(database:DatabaseConnection)-> Router{
    let cors  = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);
    let app = Router::new()
        .route("/tasks", post(create_tasks))
        .layer(Extension(database))
        .layer(cors);
    app
}