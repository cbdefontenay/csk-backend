use fs::read_to_string;
use axum::body::Body;
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::fs;
use tower_http::cors::{Any, CorsLayer};

async fn hello_world() -> &'static str {
    "Hello, world!!"
}

async fn get_translation(Path(lang): Path<String>, _headers: HeaderMap) -> impl IntoResponse {
    let lang = if lang.is_empty() {
       String::from("en")
    } else {
        lang
    };

    let path = format!("./src/translations/{}.json", lang);

    match read_to_string(path) {
        Ok(content) => (StatusCode::OK, content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Body::from("Translation not found")).into_response(),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/translations/:lang", get(get_translation))
        .layer(cors);

    Ok(router.into())
}