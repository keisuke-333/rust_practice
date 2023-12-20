use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let ip_address = "127.0.0.1";
    let port = 8080;

    let addr = SocketAddr::new(ip_address.parse().unwrap(), port);

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("👉 \x1b[94m http://localhost:{} \x1b[0m", port);

    axum::serve(listener, app).await.unwrap();
}
