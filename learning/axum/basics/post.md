# 📤 Handling POST Requests

POST requests are used to send data to the server (e.g., creating a user or a task).

## 💡 Concepts
- **`Deserialize`**: Serde trait to turn JSON into a Rust struct.
- **`Json<T>`**: Pulls the request body and parses it as `T`.
- **`StatusCode`**: Enum for HTTP status codes (e.g., `201 Created`).

## 📝 Sample Code
```rust
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateTask {
    title: String,
    priority: String,
}

#[derive(Serialize)]
struct TaskResponse {
    id: u64,
    title: String,
}

// Handler for POST
async fn create_task(
    Json(payload): Json<CreateTask>, // Extractor for Body
) -> (StatusCode, Json<TaskResponse>) {
    // Logic to save to DB here...
    
    let task = TaskResponse {
        id: 1,
        title: payload.title,
    };

    (StatusCode::CREATED, Json(task))
}

pub fn router() -> Router {
    Router::new().route("/tasks", post(create_task))
}
```

## 🔗 Resources
- [Axum JSON Documentation](https://docs.rs/axum/latest/axum/struct.Json.html)
- [Serde Overview](https://serde.rs/)
