use axum::routing::{delete, post};

#[axum::debug_handler]
async fn send_paekli() -> &'static str {
    "\
Thank you for trusting Paekli LLC!
We will deliver your paekli in mint condition.
* throws your paekli directly in the trash *"
}

#[axum::debug_handler]
async fn receive_paekli() -> &'static str {
    "\
There aren't any paekli for you at the moment.
* tries to hide paekli in the trash can *"
}

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", post(send_paekli))
        .route("/", delete(receive_paekli));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
