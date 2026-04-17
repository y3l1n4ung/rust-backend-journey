#![allow(dead_code)]
use axum::{Router, body::Body, http::Request, response::Response};
use backends::create_app;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tower::ServiceExt;
pub async fn setup_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}
pub async fn setup_app() -> Router {
    let pool = setup_db().await;
    create_app(pool)
}

pub async fn request(app: Router, method: &str, uri: &str, body: Option<&str>) -> Response<Body> {
    let mut builder = Request::builder().uri(uri).method(method);
    if body.is_some() {
        builder = builder.header("content-type", "application/json")
    }
    app.oneshot(builder.body(body.unwrap_or_default().to_string()).unwrap())
        .await
        .unwrap()
}

pub async fn body_text(res: Response<Body>) -> String {
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    String::from_utf8(bytes.to_vec()).unwrap()
}
pub async fn body_json(res: Response) -> Value {
    let text = body_text(res).await;
    serde_json::from_str(&text).unwrap()
}
