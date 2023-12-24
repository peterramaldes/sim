use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./infraestructure/database/migrations");
}

#[tokio::main]
async fn main() {
    let conn = "host=localhost user=sim password=sim dbname=sim";
    let (mut client, connection) = tokio_postgres::connect(conn, NoTls)
        .await
        .expect("cannot connect into database");

    // Spawn a new tokio runtime to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter, just to make sure
    // the database connect successfully
    client
        .query("SELECT 1", &[])
        .await
        .expect("error occurred trying to run `select 1` on postgres");

    // Run the migrations
    embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("error trying to run the migrations");

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
