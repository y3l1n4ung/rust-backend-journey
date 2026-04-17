use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: i32,
    pub completed: bool,
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreate {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdate {
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: i64,
    pub sender: String,
    pub content: String,
    pub created_at: DateTime<chrono::Utc>,
}

pub struct PaginatedResponse<T> {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
    pub data: Vec<T>,
}
