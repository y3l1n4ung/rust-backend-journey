use std::sync::{Arc, Mutex};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::axum_basic::model::{Task, TaskCreate, TaskUpdate};

pub type AppState = Arc<Mutex<Vec<Task>>>;

pub async fn get_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}
#[axum::debug_handler]
pub async fn create_tasks(
    State(state): State<AppState>,
    Json(task_create): Json<TaskCreate>,
) -> Json<Task> {
    let mut tasks = state.lock().unwrap();

    let mut task = Task {
        id: 1,
        completed: false,
        description: task_create.description,
    };
    if let Some(last) = tasks.last() {
        task.id = last.id + 1;
    }
    tasks.push(task.clone());
    Json(task)
}
#[axum::debug_handler]

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(task_update): Json<TaskUpdate>,
) -> (StatusCode, Json<Option<Task>>) {
    let mut tasks = state.lock().unwrap();
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id as i32) {
        if let Some(desc) = task_update.description {
            task.description = desc;
        }
        if let Some(completed) = task_update.completed {
            task.completed = completed;
        }
        return (StatusCode::OK, Json(Some(task.clone())));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

pub async fn delete_task(State(state): State<AppState>, Path(id): Path<i64>) -> StatusCode {
    let mut tasks = state.lock().unwrap();
    if let Some(pos) = tasks.iter().position(|t| t.id == id as i32) {
        tasks.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
