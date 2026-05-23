use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ADDR: &str = "127.0.0.1:8082";

#[derive(Clone, Debug, Serialize)]
struct HealthResponse {
    status: HealthStatus,
    service: &'static str,
    version: &'static str,
    timestamp: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum HealthStatus {
    Ok,
    Degraded,
    Unavailable,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let addr = std::env::var("ENVIRONMENT_MANAGER_ADDR")
        .unwrap_or_else(|_| DEFAULT_ADDR.to_string())
        .parse::<SocketAddr>()
        .unwrap_or_else(|error| {
            warn!(
                "invalid ENVIRONMENT_MANAGER_ADDR, defaulting to {DEFAULT_ADDR}: {error}"
            );
            DEFAULT_ADDR.parse().expect("default address invalid")
        });

    let app = Router::new().route("/health", get(health));

    info!("environment manager listening on http://{addr}");
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("failed to bind address"),
        app,
    )
    .await
    .expect("server failed");
}

async fn health() -> Json<HealthResponse> {
    let timestamp = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    Json(HealthResponse {
        status: HealthStatus::Ok,
        service: "environment-manager",
        version: "0.1.0",
        timestamp,
    })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "environment_manager=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
