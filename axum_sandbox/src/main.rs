use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler_hello))
        .layer(TraceLayer::new_for_http());

    let ip_address = "127.0.0.1";
    let port = 8080;
    let addr = SocketAddr::new(ip_address.parse().unwrap(), port);
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn handler_hello() -> impl IntoResponse {
    Html("Hello World!!!")
}
