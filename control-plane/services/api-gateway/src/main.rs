use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::{
    net::SocketAddr,
    sync::{Arc, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ADDR: &str = "127.0.0.1:8080";
const DEFAULT_REGISTRY_URL: &str = "http://127.0.0.1:8081";

#[derive(Clone)]
struct AppState {
    registry_base: String,
    client: Client,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    request_id: String,
    status: ResponseStatus,
    data: Option<T>,
    error: Option<ApiError>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ResponseStatus {
    Ok,
    Error,
}

#[derive(Serialize)]
struct ApiError {
    message: String,
}

#[derive(Serialize)]
struct HealthPayload {
    status: HealthStatus,
    service: &'static str,
    version: &'static str,
    timestamp: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum HealthStatus {
    Ok,
    Degraded,
    Unavailable,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let addr = std::env::var("API_GATEWAY_ADDR")
        .unwrap_or_else(|_| DEFAULT_ADDR.to_string())
        .parse::<SocketAddr>()
        .unwrap_or_else(|error| {
            warn!("invalid API_GATEWAY_ADDR, defaulting to {DEFAULT_ADDR}: {error}");
            DEFAULT_ADDR.parse().expect("default address invalid")
        });

    let registry_base = std::env::var("CAPABILITY_REGISTRY_URL")
        .unwrap_or_else(|_| DEFAULT_REGISTRY_URL.to_string())
        .trim_end_matches('/')
        .to_string();
    let state = Arc::new(AppState {
        registry_base,
        client: Client::new(),
    });

    let app = Router::new()
        .route("/v1/health", get(health))
        .route("/v1/capabilities", get(list_capabilities))
        .route("/v1/capabilities/:capability_id", get(get_capability))
        .with_state(state);

    info!("api gateway listening on http://{addr}");
    axum::serve(
        tokio::net::TcpListener::bind(addr)
            .await
            .expect("failed to bind address"),
        app,
    )
    .await
    .expect("server failed");
}

async fn health() -> Json<ApiResponse<HealthPayload>> {
    let timestamp = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string());
    let payload = HealthPayload {
        status: HealthStatus::Ok,
        service: "api-gateway",
        version: "0.1.0",
        timestamp,
    };
    Json(ApiResponse::ok(payload))
}

async fn list_capabilities(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match fetch_registry(&state, "/v1/capabilities").await {
        Ok(payload) => Json(ApiResponse::ok(payload)).into_response(),
        Err((status, message)) => error_response(status, message).into_response(),
    }
}

async fn get_capability(
    State(state): State<Arc<AppState>>,
    Path(capability_id): Path<String>,
) -> impl IntoResponse {
    let path = format!("/v1/capabilities/{capability_id}");
    match fetch_registry(&state, &path).await {
        Ok(payload) => Json(ApiResponse::ok(payload)).into_response(),
        Err((status, message)) => error_response(status, message).into_response(),
    }
}

async fn fetch_registry(state: &AppState, path: &str) -> Result<Value, (StatusCode, String)> {
    let url = format!("{}{}", state.registry_base, path);
    let response = state
        .client
        .get(url)
        .send()
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, format!("registry error: {error}")))?;

    let status = response.status();
    if !status.is_success() {
        let message = if status == StatusCode::NOT_FOUND {
            "capability not found".to_string()
        } else {
            format!("registry returned {status}")
        };
        return Err((status, message));
    }

    response
        .json::<Value>()
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, format!("invalid registry payload: {error}")))
}

fn error_response(status: StatusCode, message: String) -> (StatusCode, Json<ApiResponse<Value>>) {
    (
        status,
        Json(ApiResponse::error(ApiError { message })),
    )
}

impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        Self {
            request_id: next_request_id(),
            status: ResponseStatus::Ok,
            data: Some(data),
            error: None,
        }
    }

    fn error(error: ApiError) -> Self {
        Self {
            request_id: next_request_id(),
            status: ResponseStatus::Error,
            data: None,
            error: Some(error),
        }
    }
}

fn next_request_id() -> String {
    static COUNTER: OnceLock<std::sync::atomic::AtomicU64> = OnceLock::new();
    let counter = COUNTER.get_or_init(|| std::sync::atomic::AtomicU64::new(0));
    let sequence = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("req-{now}-{sequence}")
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
