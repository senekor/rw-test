use std::sync::Mutex;

use axum::{
    http::StatusCode,
    routing::{delete, post},
    Json,
};
use serde::{Deserialize, Serialize};

static PAEKLI_STORE: Mutex<Option<String>> = Mutex::new(None);

#[derive(Deserialize)]
struct SendRequest {
    content: String,
}

#[axum::debug_handler]
async fn send_paekli(Json(request): Json<SendRequest>) {
    let mut guard = PAEKLI_STORE.lock().unwrap();
    *guard = Some(request.content);
}

#[derive(Serialize)]
struct ReceiveResponse {
    content: String,
}

#[axum::debug_handler]
async fn receive_paekli() -> Result<Json<ReceiveResponse>, StatusCode> {
    let mut guard = PAEKLI_STORE.lock().unwrap();
    match guard.take() {
        Some(content) => Ok(Json(ReceiveResponse { content })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", post(send_paekli))
        .route("/", delete(receive_paekli));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
