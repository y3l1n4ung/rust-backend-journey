use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

use crate::axum_basic::model::{Task, TaskCreate, TaskUpdate};

pub type AppState = SqlitePool;

pub async fn get_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode> {
    let tasks =
        sqlx::query_as::<_, Task>("SELECT id, completed, description FROM tasks ORDER by id DESC")
            .fetch_all(&state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tasks))
}
#[axum::debug_handler]
pub async fn create_tasks(
    State(state): State<AppState>,
    Json(task_create): Json<TaskCreate>,
) -> Result<(StatusCode, Json<Task>), StatusCode> {
    let result = sqlx::query("INSERT INTO tasks (description, completed) VALUES (?,?)")
        .bind(task_create.description)
        .bind(false)
        .execute(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = result.last_insert_rowid();

    let task =
        sqlx::query_as::<_, Task>("SELECT id, completed, description FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_one(&state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok((StatusCode::CREATED, Json(task)))
}

#[axum::debug_handler]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(task_update): Json<TaskUpdate>,
) -> Result<Json<Task>, StatusCode> {
    let existing =
        sqlx::query_as::<_, Task>("SELECT id, completed, description FROM tasks WHERE id = ?")
            .bind(id)
            .fetch_optional(&state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if let Some(task) = existing {
        let description = task_update.description.unwrap_or(task.description);
        let completed = task_update.completed.unwrap_or(task.completed);
        sqlx::query("UPDATE tasks SET description = ? , completed = ? WHERE id = ?")
            .bind(description)
            .bind(completed)
            .bind(id)
            .execute(&state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let updated =
            sqlx::query_as::<_, Task>("SELECT id, completed, description FROM tasks WHERE id = ?")
                .bind(id)
                .fetch_one(&state)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(updated));
    }

    Err(StatusCode::NOT_FOUND)
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let deleted = sqlx::query("DELETE from tasks where id = ?")
        .bind(id)
        .execute(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if deleted.rows_affected() > 0 {
        return Ok(StatusCode::NO_CONTENT);
    }

    Ok(StatusCode::NOT_FOUND)
}
