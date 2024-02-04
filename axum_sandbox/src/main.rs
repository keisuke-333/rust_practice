use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler_hello))
        .route("/users", post(create_user))
        .layer(TraceLayer::new_for_http());

    let ip_address = "127.0.0.1";
    let port = 8080;
    let addr = SocketAddr::new(ip_address.parse().unwrap(), port);
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    Html("Hello World!!!")
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 13,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
