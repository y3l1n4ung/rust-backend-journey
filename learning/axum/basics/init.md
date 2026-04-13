# 🚀 Axum Initialization (2026)

To start a Rust backend, you need an **Async Runtime** (Tokio) and **Observability** (Tracing).

## 💡 Concepts
- **`#[tokio::main]`**: The entry point for async execution.
- **`tracing-subscriber`**: The standard way to see logs/traces in the terminal.
- **`Router`**: The core of Axum where you define paths.

## 📝 Sample Code
```rust
use axum::{Router, routing::get};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 1. Initialize Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Build Router
    let app = Router::new().route("/", get(|| async { "Hello World" }));

    // 3. Start Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

## 🔗 Resources
- [Axum Official Docs](https://docs.rs/axum/latest/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Tracing Subscriber Guide](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
