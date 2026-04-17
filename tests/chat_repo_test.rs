mod helper;

use backends::axum_basic::chat_repo::{
    create_message, delete_message, get_message_by_id, list_message,
};

use helper::setup_db;
#[tokio::test]
async fn test_create_message_and_get_message() {
    let pool = setup_db().await;
    let id = create_message(&pool, "human", "How are you?")
        .await
        .unwrap();

    let message = get_message_by_id(&pool, id).await.unwrap().unwrap();

    assert_eq!(message.id, id);
    assert_eq!(message.sender, "human");
    assert_eq!(message.content, "How are you?")
}

#[tokio::test]
async fn test_list_message_pagination() {
    let pool = setup_db().await;

    // Initial greeting
    create_message(&pool, "ASSISTANT", "How can I help you today?")
        .await
        .unwrap();

    // Human inquiry
    create_message(
        &pool,
        "HUMAN",
        "Hi, I'm looking for some help with my Rust project.",
    )
    .await
    .unwrap();

    // Assistant follow-up
    create_message(
        &pool,
        "ASSISTANT",
        "I'd be happy to help! What specifically are you working on?",
    )
    .await
    .unwrap();

    // Technical context
    create_message(
        &pool,
        "HUMAN",
        "I'm trying to implement a paginated chat history using SQLx and SQLite.",
    )
    .await
    .unwrap();

    // Specific troubleshooting
    create_message(
        &pool,
        "ASSISTANT",
        "That's a great choice. Are you using `FromRow` to map your database results?",
    )
    .await
    .unwrap();

    create_message(
        &pool,
        "HUMAN",
        "Yes, but I ran into some issues with the `created_at` field and sorting.",
    )
    .await
    .unwrap();

    create_message(&pool, "ASSISTANT", "Sorting can be tricky! Usually, you want `ORDER BY created_at ASC` for standard chat flow.").await.unwrap();

    // Resolution
    create_message(
        &pool,
        "HUMAN",
        "Thanks! I'll try that. How should I handle the WebSocket conversion?",
    )
    .await
    .unwrap();

    create_message(&pool, "ASSISTANT", "You'll need to serialize your struct to JSON and wrap it in `axum::extract::ws::Message::Text`.").await.unwrap();

    create_message(
        &pool,
        "HUMAN",
        "Perfect, it's working now. Thanks for the help!",
    )
    .await
    .unwrap();

    create_message(
        &pool,
        "ASSISTANT",
        "You're very welcome! Let me know if you have any other questions.",
    )
    .await
    .unwrap();

    let res = list_message(&pool, 10, 0).await.unwrap();

    assert_eq!(res.limit, 10);
    assert_eq!(res.offset, 0);
    assert_eq!(res.data.len(), 10);
    assert_eq!(res.total, 11);
}

#[tokio::test]
async fn test_delete_message() {
    let pool = setup_db().await;
    let id = create_message(&pool, "human", "How are you?")
        .await
        .unwrap();
    let message = get_message_by_id(&pool, id).await.unwrap().unwrap();

    assert_eq!(message.id, id);
    assert_eq!(message.sender, "human");
    assert_eq!(message.content, "How are you?");

    let r = delete_message(&pool, id).await.unwrap();
    assert_eq!(r, 1);

    let message = get_message_by_id(&pool, id).await.unwrap();
    assert!(message.is_none());
}
