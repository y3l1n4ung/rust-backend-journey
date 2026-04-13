# 📥 Handling GET Requests

GET requests are used to retrieve data. In Axum, we use **Extractors** to pull data from the URL.

## 💡 Concepts
- **`Path<T>`**: Pulls variables directly from the URL (e.g., `/user/:id`).
- **`Json<T>`**: Automatically serializes structs into JSON.
- **`IntoResponse`**: Trait that allows you to return any type (String, Status Code, Json).

## 📝 Sample Code
```rust
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// 1. Static GET
async fn hello() -> &'static str {
    "Hello User!"
}

// 2. Path Extractors
async fn get_user(Path(user_id): Path<u64>) -> impl IntoResponse {
    let user = User {
        id: user_id,
        username: "ferris".into(),
    };
    Json(user)
}

pub fn router() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/users/:id", get(get_user)) // :id is the key
}
```

## 🔗 Resources
- [Axum Extractors Guide](https://docs.rs/axum/latest/axum/extract/index.html)
- [Path Extractor Documentation](https://docs.rs/axum/latest/axum/extract/struct.Path.html)
