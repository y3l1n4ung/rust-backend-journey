# ❓ Query Parameters

Query parameters are used for filtering, sorting, or pagination (e.g., `/tasks?status=completed`).

## 💡 Concepts
- **`Query<T>`**: Pulls search/filter parameters from the URL after `?`.
- **`Option<T>`**: Use this for optional parameters that might not be in the URL.

## 📝 Sample Code
```rust
use axum::{
    extract::Query,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Pagination {
    page: Option<u64>,
    per_page: Option<u64>,
    status: Option<String>,
}

#[derive(Serialize)]
struct Task {
    id: u64,
    status: String,
}

// Handler with Query params
async fn list_tasks(
    Query(params): Query<Pagination>, // Extractor for ?page=1
) -> Json<Vec<Task>> {
    let page = params.page.unwrap_or(1);
    let status = params.status.unwrap_or_else(|| "all".to_string());
    
    tracing::info!("Listing tasks for page {} with status {}", page, status);

    let tasks = vec![
        Task { id: 1, status: "completed".into() },
        Task { id: 2, status: "pending".into() },
    ];

    Json(tasks)
}

pub fn router() -> Router {
    Router::new().route("/tasks", get(list_tasks))
}
```

## 🔗 Resources
- [Axum Query Documentation](https://docs.rs/axum/latest/axum/extract/struct.Query.html)
- [Serde Deserialization Attributes](https://serde.rs/attributes.html)
