use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_ADDR: &str = "127.0.0.1:8081";

#[derive(Clone, Debug, Serialize)]
struct HealthResponse {
    status: HealthStatus,
    service: &'static str,
    version: &'static str,
    timestamp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum HealthStatus {
    Ok,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CapabilityClass {
    Cpu,
    Gpu,
    Io,
    Memory,
    Network,
    Storage,
    Accelerator,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum CapabilityValue {
    Bool(bool),
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Capability {
    id: String,
    class: CapabilityClass,
    version: String,
    health: HealthStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    revision: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    limits: BTreeMap<String, CapabilityValue>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    properties: BTreeMap<String, CapabilityValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    labels: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CapabilitySnapshot {
    generated_at: u64,
    capabilities: Vec<Capability>,
}

#[derive(Clone, Debug, Serialize)]
struct RegisterResponse {
    accepted: usize,
    generated_at: u64,
}

#[derive(Debug, Default)]
struct RegistryState {
    capabilities: HashMap<String, Capability>,
    last_generated_at: Option<u64>,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let addr = std::env::var("CAPABILITY_REGISTRY_ADDR")
        .unwrap_or_else(|_| DEFAULT_ADDR.to_string())
        .parse::<SocketAddr>()
        .unwrap_or_else(|error| {
            warn!(
                "invalid CAPABILITY_REGISTRY_ADDR, defaulting to {DEFAULT_ADDR}: {error}"
            );
            DEFAULT_ADDR.parse().expect("default address invalid")
        });

    let state = Arc::new(RwLock::new(RegistryState::default()));
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/capabilities", get(list_capabilities))
        .route("/v1/capabilities/register", post(register_capabilities))
        .route("/v1/capabilities/:capability_id", get(get_capability))
        .with_state(state);

    info!("capability registry listening on http://{addr}");
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
        service: "capability-registry",
        version: "0.1.0",
        timestamp,
    })
}

async fn list_capabilities(
    State(state): State<Arc<RwLock<RegistryState>>>,
) -> Json<Vec<Capability>> {
    let guard = state.read().expect("registry lock poisoned");
    let mut capabilities: Vec<Capability> = guard.capabilities.values().cloned().collect();
    capabilities.sort_by(|left, right| left.id.cmp(&right.id));
    Json(capabilities)
}

async fn get_capability(
    State(state): State<Arc<RwLock<RegistryState>>>,
    Path(capability_id): Path<String>,
) -> Result<Json<Capability>, StatusCode> {
    let guard = state.read().expect("registry lock poisoned");
    guard
        .capabilities
        .get(&capability_id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn register_capabilities(
    State(state): State<Arc<RwLock<RegistryState>>>,
    Json(snapshot): Json<CapabilitySnapshot>,
) -> Json<RegisterResponse> {
    let mut guard = state.write().expect("registry lock poisoned");
    let accepted = snapshot.capabilities.len();
    for capability in snapshot.capabilities {
        guard.capabilities.insert(capability.id.clone(), capability);
    }
    guard.last_generated_at = Some(snapshot.generated_at);
    Json(RegisterResponse {
        accepted,
        generated_at: snapshot.generated_at,
    })
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "capability_registry=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
