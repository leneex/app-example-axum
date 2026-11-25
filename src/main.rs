use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(hello))
        .route("/echo", post(echo));

    // The platform passes the port in the PORT variable; 5000 by default
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(5000);

    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello world"
}

async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

#[derive(Deserialize)]
struct EchoIn {
    message: String,
}

#[derive(Serialize)]
struct EchoOut {
    echoed: String,
}

async fn echo(Json(payload): Json<EchoIn>) -> Json<EchoOut> {
    Json(EchoOut {
        echoed: payload.message,
    })
}
