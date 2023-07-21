use axum::routing::{get, post};
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/tags", get(tags))
        .route("/api/users", post(create_user));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn tags() -> &'static str {
    "Tags!"
}

async fn create_user() -> &'static str {
    "Create User"
}
