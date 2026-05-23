use crate::{Capability, CapabilityClass, CapabilitySnapshot, CapabilityValue, HealthStatus};
use serde_json::{Map, Number, Value};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn control_plane_register_payload(snapshot: &CapabilitySnapshot) -> Value {
    let capabilities = snapshot
        .capabilities
        .iter()
        .map(control_plane_capability)
        .collect::<Vec<_>>();
    let mut payload = Map::new();
    payload.insert(
        "generated_at".to_string(),
        Value::Number(Number::from(system_time_to_epoch_seconds(snapshot.generated_at))),
    );
    payload.insert("capabilities".to_string(), Value::Array(capabilities));
    Value::Object(payload)
}

impl CapabilitySnapshot {
    pub fn to_control_plane_register(&self) -> Value {
        control_plane_register_payload(self)
    }
}

fn system_time_to_epoch_seconds(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn control_plane_capability(capability: &Capability) -> Value {
    let mut payload = Map::new();
    payload.insert("id".to_string(), Value::String(capability.id.clone()));
    payload.insert(
        "class".to_string(),
        Value::String(capability_class_name(&capability.class).to_string()),
    );
    payload.insert(
        "version".to_string(),
        Value::String(capability.version.clone()),
    );
    payload.insert(
        "health".to_string(),
        Value::String(health_status_name(&capability.health).to_string()),
    );
    insert_optional_text(&mut payload, "vendor", capability.vendor.as_ref());
    insert_optional_text(&mut payload, "model", capability.model.as_ref());
    insert_optional_text(&mut payload, "revision", capability.revision.as_ref());
    insert_optional_text(&mut payload, "location", capability.location.as_ref());

    if !capability.limits.is_empty() {
        payload.insert(
            "limits".to_string(),
            Value::Object(capability_map(&capability.limits)),
        );
    }

    if !capability.properties.is_empty() {
        payload.insert(
            "properties".to_string(),
            Value::Object(capability_map(&capability.properties)),
        );
    }

    if !capability.labels.is_empty() {
        payload.insert(
            "labels".to_string(),
            Value::Array(
                capability
                    .labels
                    .iter()
                    .map(|label| Value::String(label.clone()))
                    .collect(),
            ),
        );
    }

    Value::Object(payload)
}

fn capability_map(values: &BTreeMap<String, CapabilityValue>) -> Map<String, Value> {
    values
        .iter()
        .map(|(key, value)| (key.clone(), capability_value(value)))
        .collect()
}

fn capability_value(value: &CapabilityValue) -> Value {
    match value {
        CapabilityValue::Bool(value) => Value::Bool(*value),
        CapabilityValue::Number(value) => Number::from_f64(*value)
            .map(Value::Number)
            .unwrap_or_else(|| Value::String(value.to_string())),
        CapabilityValue::Text(value) => Value::String(value.clone()),
    }
}

fn capability_class_name(class: &CapabilityClass) -> &'static str {
    match class {
        CapabilityClass::Cpu => "cpu",
        CapabilityClass::Gpu => "gpu",
        CapabilityClass::Io => "io",
        CapabilityClass::Memory => "memory",
        CapabilityClass::Network => "network",
        CapabilityClass::Storage => "storage",
        CapabilityClass::Accelerator => "accelerator",
    }
}

fn health_status_name(status: &HealthStatus) -> &'static str {
    match status {
        HealthStatus::Ok => "ok",
        HealthStatus::Degraded => "degraded",
        HealthStatus::Unavailable => "unavailable",
    }
}

fn insert_optional_text(payload: &mut Map<String, Value>, key: &str, value: Option<&String>) {
    if let Some(value) = value {
        payload.insert(key.to_string(), Value::String(value.clone()));
    }
}
