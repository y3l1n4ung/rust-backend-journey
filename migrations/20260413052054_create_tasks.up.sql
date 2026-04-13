-- Add up migration script here
CREATE TABLE tasks (
 id INTEGER PRIMARY KEY AUTOINCREMENT,
 completed BOOLEAN NOT NULL DEFAULT 0,
 description TEXT NOT NULL
);

CREATE INDEX idx_tasks_completed ON tasks (completed);
