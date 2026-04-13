use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
