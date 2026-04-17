use anyhow::Ok;
use sqlx::SqlitePool;

use crate::axum_basic::model::{ChatMessage, PaginatedResponse};

pub async fn create_message(pool: &SqlitePool, sender: &str, content: &str) -> anyhow::Result<i64> {
    let result = sqlx::query("INSERT  INTO chat_messages (sender, content) VALUES (?, ?)")
        .bind(sender)
        .bind(content)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_message_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<ChatMessage>> {
    let message = sqlx::query_as::<_, ChatMessage>(
        "SELECT id, sender, content, created_at FROM chat_messages WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(message)
}

pub async fn list_message(
    pool: &SqlitePool,
    limit: i64,
    offset: i64,
) -> anyhow::Result<PaginatedResponse<ChatMessage>> {
    let data = sqlx::query_as::<_,ChatMessage>("SELECT id, sender, content, created_at FROM chat_messages ORDER BY id DESC LIMIT ? OFFSET ? ")
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chat_messages")
        .fetch_one(pool)
        .await?;

    Ok(PaginatedResponse {
        limit,
        offset,
        total,
        data,
    })
}

pub async fn delete_message(pool: &SqlitePool, id: i64) -> anyhow::Result<u64> {
    let result = sqlx::query("DELETE FROM chat_messages WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
