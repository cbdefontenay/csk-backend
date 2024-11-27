use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

async fn hello_world() -> &'static str {
    "Hello, world!!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/hello-world", get(hello_world))
        .layer(cors);

    Ok(router.into())
}
