use std::time::Duration;

use backends::create_app;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Span;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false) // Removes 'tower_http::trace::on_response'
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE) // Removes 'request:'
        .compact()
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:tasks.db")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            // Capture details once; we use an empty span name "" to hide "request:"
            tracing::info_span!(
                "",
                method = %request.method(),
                uri = %request.uri(),
                status = tracing::field::Empty // Placeholder for the response
            )
        })
        .on_request(()) // Disable the "started processing" log line
        .on_response(
            |res: &axum::http::Response<_>, latency: Duration, span: &Span| {
                let status = res.status().as_u16();
                let latency = latency.as_millis();

                // record() adds the status to the existing span fields (method, uri)
                span.record("status", status);

                // This prints a single clean line: INFO 200 GET /tasks 5ms
                tracing::info!("{}ms", latency);
            },
        );

    let app = create_app(pool).layer(trace_layer);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
