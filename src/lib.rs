use sea_orm::Database;
mod routes;
mod database;
pub async fn run(database_uri:String){
    
    let database = Database::connect(database_uri).await.unwrap();
    println!("Starting Server at localhost:3000");
    let app = routes::create_routes(database);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}