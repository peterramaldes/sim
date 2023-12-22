use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/simulate", post(simulate));

    // TODO: add the port on some environment variable
    let port = 3000.to_string();

    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn simulate() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
