# 🚀 Rust Backend Learning TODO Plan 2026

A step-by-step plan to become an industry-ready Rust backend developer.

## Phase 1: Foundations (Week 1-2)
- [ ] **Modern Rust Fundamentals**: Review 2024 Edition features.
- [ ] **Error Handling**: Implement custom error types with `thiserror`.
- [ ] **Async Basics**: Build a simple `tokio` multi-threaded task runner.
- [ ] **Smart Pointers**: Practical usage of `Arc<Mutex<T>>` and `RwLock`.

## Phase 2: The Web API (Week 3-4)
- [ ] **Hello Axum**: Build a basic REST API with static routes.
- [ ] **Extractors**: Use `Path`, `Query`, and `Json` extractors.
- [ ] **Middleware**: Implement a custom "Tower Layer" for request logging.
- [ ] **Validation**: Integrate `validator` crate for type-safe JSON payloads.

## Phase 3: The Data Layer (Week 5-6)
- [ ] **SQLx Setup**: Connect to PostgreSQL and perform basic CRUD.
- [ ] **Query Macros**: Use `sqlx::query!` for compile-time safety.
- [ ] **Migrations**: Automate schema changes with `sqlx-cli`.
- [ ] **Concurrency**: Use `PgPool` for high-performance connection pooling.

## Phase 4: Observability (Week 7)
- [ ] **Tracing Layers**: Setup `tracing-subscriber` with `EnvFilter`.
- [ ] **Instrumentation**: Add `#[tracing::instrument]` to important functions.
- [ ] **OTel Export**: Export traces to a local Jaeger/Grafana instance.

## Phase 5: Security & Auth (Week 8)
- [ ] **PASETO Implementation**: Build a secure token-based auth flow.
- [ ] **Hashing**: Use `argon2` for password hashing.
- [ ] **Auth Layers**: Write a Tower middleware to protect private routes.

## Phase 6: Scaling & Communication (Week 9-10)
- [ ] **gRPC Service**: Build a companion service with `Tonic`.
- [ ] **Redis**: Implement a distributed cache for frequently accessed data.
- [ ] **Testing**: Write integration tests using `testcontainers-rs`.

## Phase 7: Deployment & Optimization (Week 11-12)
- [ ] **Dockerization**: Create a distroless Docker image.
- [ ] **Profiling**: Identify bottlenecks with `cargo-flamegraph`.
- [ ] **CI/CD**: Setup GitHub Actions with `cargo-audit` and `cargo-nextest`.
