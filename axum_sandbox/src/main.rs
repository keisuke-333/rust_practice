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

    let app = create_app();

    let ip_address = "127.0.0.1";
    let port = 8080;
    let addr = SocketAddr::new(ip_address.parse().unwrap(), port);
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

fn create_app() -> Router {
    Router::new()
        .route("/", get(handler_hello))
        .route("/users", post(create_user))
        .layer(TraceLayer::new_for_http())
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

#[derive(Serialize,Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize,Deserialize,Debug,PartialEq,Eq)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use tower::ServiceExt;
    use http_body_util::BodyExt; 

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder()
            .uri("/")
            .body(Body::empty())
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello World!!!");
    }

    #[tokio::test]
    async fn should_return_user_data() {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{ "username": "テスト 太郎" }"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user: User = serde_json::from_str(&body).expect("cannot convert User instance.");
        assert_eq!(user, User { id: 13, username: "テスト 太郎".to_string(), });
    }
}