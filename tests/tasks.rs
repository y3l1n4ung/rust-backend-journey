use axum::http::StatusCode;
use serde_json::Value;

mod helper;
use helper::request;

#[tokio::test]
async fn test_get_tasks() {
    let app = helper::setup_app().await;
    let res = request(app, "GET", "/tasks", None).await;
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(helper::body_text(res).await, "[]");
}

#[tokio::test]
async fn test_create_task_success() {
    let app = helper::setup_app().await;
    let res = request(app, "POST", "/tasks", Some(r#"{"description":"task1"}"#)).await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = helper::body_text(res).await;
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(json["id"], 1);
    assert_eq!(json["description"], "task1");
    assert_eq!(json["completed"], false);
}

#[tokio::test]
async fn test_get_tasks_after_create() {
    let app = helper::setup_app().await;
    let res = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(r#"{"description":"task1"}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let json: Value = helper::body_json(res).await;

    assert_eq!(json["id"], 1);
    assert_eq!(json["description"], "task1");
    assert_eq!(json["completed"], false);

    let res = request(app, "GET", "/tasks", None).await;
    assert_eq!(res.status(), StatusCode::OK);

    let json: Value = helper::body_json(res).await;
    assert_eq!(json.as_array().unwrap().len(), 1);
    assert_eq!(json[0]["description"], "task1");
    assert_eq!(json[0]["completed"], false);
}

#[tokio::test]
async fn test_update_tasks_after_create() {
    let app = helper::setup_app().await;
    let res = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(r#"{"description":"task1"}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::CREATED);

    let json: Value = helper::body_json(res).await;

    assert_eq!(json["id"], 1);
    assert_eq!(json["description"], "task1");
    assert_eq!(json["completed"], false);

    let res = request(
        app.clone(),
        "PATCH",
        "/tasks/1",
        Some(r#"{"completed":true}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK,);

    let updated: Value = helper::body_json(res).await;
    assert_eq!(updated["id"], 1);
    assert_eq!(updated["description"], "task1");
    assert_eq!(updated["completed"], true);

    // update both
    let res = request(
        app,
        "PATCH",
        "/tasks/1",
        Some(r#"{"completed":false,"description":"task1#updated"}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::OK,);
    let updated: Value = helper::body_json(res).await;

    assert_eq!(updated["id"], 1);
    assert_eq!(updated["description"], "task1#updated");
    assert_eq!(updated["completed"], false);
}

#[tokio::test]
async fn test_update_task_not_found() {
    let app = helper::setup_app().await;

    let res = request(
        app.clone(),
        "PATCH",
        "/tasks/1",
        Some(r#"{"completed":true}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND,);
}
#[tokio::test]
async fn test_update_task_invalid_body() {
    let app = helper::setup_app().await;

    let res = request(app.clone(), "PATCH", "/tasks/1", Some(r#"{"#)).await;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST,);
}

#[tokio::test]
async fn test_delete_task_success() {
    let app = helper::setup_app().await;
    let res = request(
        app.clone(),
        "POST",
        "/tasks",
        Some(r#"{"description":"task1"}"#),
    )
    .await;
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = helper::body_text(res).await;
    let json: Value = serde_json::from_str(&body).unwrap();

    assert_eq!(json["id"], 1);
    assert_eq!(json["description"], "task1");
    assert_eq!(json["completed"], false);

    let res = request(app.clone(), "DELETE", "/tasks/1", None).await;
    assert_eq!(res.status(), StatusCode::NO_CONTENT,);

    let res = request(app, "DELETE", "/tasks/1", None).await;
    assert_eq!(res.status(), StatusCode::NOT_FOUND,);
}
