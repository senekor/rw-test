use axum::routing::get;

async fn hello_world() -> &'static str {
    "hello world"
}

#[tokio::main]
async fn main() {
    let router = axum::Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
