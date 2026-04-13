use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::axum_basic::handler::{AppState, create_tasks, delete_task, get_tasks, update_task};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "ok" }))
        .route("/tasks", get(get_tasks))
        .route("/tasks", post(create_tasks))
        .route("/tasks/{id}", patch(update_task).delete(delete_task))
        .with_state(state)
}
