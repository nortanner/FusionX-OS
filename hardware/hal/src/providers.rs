use crate::models::{CapabilitySnapshot, CapabilityValue};
use crate::{Capability, CapabilityClass, HealthStatus};
#[cfg(target_os = "macos")]
use std::process::Command;
use std::time::SystemTime;
use serde_json::Value;

pub trait CapabilityProvider {
    fn name(&self) -> &str;
    fn snapshot(&self) -> CapabilitySnapshot;
}

#[derive(Default)]
pub struct AppleSiliconProvider;

impl AppleSiliconProvider {
    pub fn new() -> Self {
        Self
    }
}

impl CapabilityProvider for AppleSiliconProvider {
    fn name(&self) -> &str {
        "apple-silicon"
    }

    fn snapshot(&self) -> CapabilitySnapshot {
        let mut capabilities = Vec::new();

        let mut cpu = Capability::new("cpu.cluster0", CapabilityClass::Cpu, "v1");
        let is_macos = cfg!(target_os = "macos");
        if is_macos {
            cpu.vendor = Some("Apple".to_string());
            cpu.location = Some("soc".to_string());
        }
        cpu.model = sysctl_text("hw.model");

        if let Some(core_count) = sysctl_number("hw.ncpu") {
            cpu.properties.insert(
                "core_count".to_string(),
                CapabilityValue::Number(core_count),
            );
        }

        if let Some(arch) = sysctl_text("hw.machine") {
            cpu.properties
                .insert("arch".to_string(), CapabilityValue::Text(arch));
        }

        if cpu.properties.is_empty() {
            cpu.health = HealthStatus::Degraded;
        }
        capabilities.push(cpu);

        let mut memory = Capability::new("memory.unified", CapabilityClass::Memory, "v1");
        if is_macos {
            memory.vendor = Some("Apple".to_string());
            memory.location = Some("soc".to_string());
        }

        if let Some(total_bytes) = sysctl_number("hw.memsize") {
            memory.properties.insert(
                "total_bytes".to_string(),
                CapabilityValue::Number(total_bytes),
            );
        }

        if let Some(page_size) = sysctl_number("hw.pagesize") {
            memory.properties.insert(
                "page_size_bytes".to_string(),
                CapabilityValue::Number(page_size),
            );
        }

        if memory.properties.is_empty() {
            memory.health = HealthStatus::Degraded;
        }
        capabilities.push(memory);

        if is_macos {
            let mut gpu_capabilities = apple_silicon_gpu_capabilities();
            if gpu_capabilities.is_empty() {
                let mut gpu = Capability::new("gpu.unknown", CapabilityClass::Gpu, "v1");
                gpu.vendor = Some("Apple".to_string());
                gpu.location = Some("soc".to_string());
                gpu.health = HealthStatus::Degraded;
                gpu_capabilities.push(gpu);
            }
            capabilities.extend(gpu_capabilities);
        }

        CapabilitySnapshot {
            generated_at: SystemTime::now(),
            capabilities,
        }
    }
}

fn apple_silicon_gpu_capabilities() -> Vec<Capability> {
    let Some(profile) = system_profiler_json() else {
        return Vec::new();
    };
    let displays = profile
        .get("SPDisplaysDataType")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();

    let mut results = Vec::new();
    for (index, entry) in displays.into_iter().enumerate() {
        let name = entry
            .get("_name")
            .and_then(|value| value.as_str())
            .unwrap_or("Apple GPU");
        let mut gpu = Capability::new(format!("gpu.{index}"), CapabilityClass::Gpu, "v1");
        gpu.vendor = Some("Apple".to_string());
        gpu.model = Some(name.to_string());
        gpu.location = Some("soc".to_string());
        gpu.labels
            .extend(vec!["integrated".to_string(), "apple-silicon".to_string()]);

        insert_metal_properties(&mut gpu, entry.get("spdisplays_metal"));
        insert_vram_shared_properties(&mut gpu, entry.get("spdisplays_vram_shared"));
        insert_vram_properties(&mut gpu, entry.get("spdisplays_vram"));
        insert_text_property(&mut gpu, "device_id", entry.get("spdisplays_device-id"));
        insert_text_property(&mut gpu, "vendor_id", entry.get("spdisplays_vendor-id"));
        insert_text_property(&mut gpu, "revision_id", entry.get("spdisplays_revision-id"));

        if gpu.properties.is_empty() {
            gpu.health = HealthStatus::Degraded;
        }
        results.push(gpu);
    }

    results
}

#[cfg(target_os = "macos")]
fn sysctl_value(key: &str) -> Option<String> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg(key)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

#[cfg(not(target_os = "macos"))]
fn sysctl_value(_key: &str) -> Option<String> {
    None
}

fn sysctl_number(key: &str) -> Option<f64> {
    sysctl_value(key)?.parse::<f64>().ok()
}

fn sysctl_text(key: &str) -> Option<String> {
    sysctl_value(key)
}

#[cfg(target_os = "macos")]
fn system_profiler_json() -> Option<Value> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .arg("-json")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    serde_json::from_slice(&output.stdout).ok()
}

#[cfg(not(target_os = "macos"))]
fn system_profiler_json() -> Option<Value> {
    None
}

fn insert_text_property(target: &mut Capability, key: &str, value: Option<&Value>) {
    let Some(value) = value else {
        return;
    };
    if let Some(text) = value.as_str() {
        target.properties.insert(
            key.to_string(),
            CapabilityValue::Text(text.to_string()),
        );
    } else if let Some(number) = value.as_f64() {
        target
            .properties
            .insert(key.to_string(), CapabilityValue::Number(number));
    }
}

fn insert_metal_properties(target: &mut Capability, value: Option<&Value>) {
    let Some(value) = value else {
        return;
    };
    match value {
        Value::Bool(value) => {
            target
                .properties
                .insert("metal_support".to_string(), CapabilityValue::Bool(*value));
        }
        Value::Number(number) => {
            if let Some(value) = number.as_f64() {
                target.properties.insert(
                    "metal_support".to_string(),
                    CapabilityValue::Number(value),
                );
            }
        }
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Some(supported) = parse_metal_support(trimmed) {
                target.properties.insert(
                    "metal_support".to_string(),
                    CapabilityValue::Bool(supported),
                );
                if let Some(feature_set) = parse_metal_feature_set(trimmed) {
                    target.properties.insert(
                        "metal_feature_set".to_string(),
                        CapabilityValue::Text(feature_set),
                    );
                } else if trimmed.to_lowercase() != "supported" {
                    target.properties.insert(
                        "metal_support_detail".to_string(),
                        CapabilityValue::Text(trimmed.to_string()),
                    );
                }
            } else {
                target.properties.insert(
                    "metal_support_detail".to_string(),
                    CapabilityValue::Text(trimmed.to_string()),
                );
            }
        }
        _ => {}
    }
}

fn insert_vram_properties(target: &mut Capability, value: Option<&Value>) {
    let Some(value) = value else {
        return;
    };
    match value {
        Value::Number(number) => {
            if let Some(value) = number.as_f64() {
                target
                    .properties
                    .insert("vram_bytes".to_string(), CapabilityValue::Number(value));
            }
        }
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Some(bytes) = parse_memory_bytes(trimmed) {
                target
                    .properties
                    .insert("vram_bytes".to_string(), CapabilityValue::Number(bytes));
            } else {
                target
                    .properties
                    .insert("vram".to_string(), CapabilityValue::Text(trimmed.to_string()));
            }
        }
        _ => {}
    }
}

fn insert_vram_shared_properties(target: &mut Capability, value: Option<&Value>) {
    let Some(value) = value else {
        return;
    };
    match value {
        Value::Bool(value) => {
            target
                .properties
                .insert("vram_shared".to_string(), CapabilityValue::Bool(*value));
        }
        Value::Number(number) => {
            if let Some(value) = number.as_f64() {
                target.properties.insert(
                    "vram_shared_bytes".to_string(),
                    CapabilityValue::Number(value),
                );
                target
                    .properties
                    .insert("vram_shared".to_string(), CapabilityValue::Bool(true));
            }
        }
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Some(bytes) = parse_memory_bytes(trimmed) {
                target.properties.insert(
                    "vram_shared_bytes".to_string(),
                    CapabilityValue::Number(bytes),
                );
                target
                    .properties
                    .insert("vram_shared".to_string(), CapabilityValue::Bool(true));
            } else if let Some(shared) = parse_shared_flag(trimmed) {
                target
                    .properties
                    .insert("vram_shared".to_string(), CapabilityValue::Bool(shared));
            } else {
                target.properties.insert(
                    "vram_shared_detail".to_string(),
                    CapabilityValue::Text(trimmed.to_string()),
                );
            }
        }
        _ => {}
    }
}

fn parse_memory_bytes(text: &str) -> Option<f64> {
    let mut number = String::new();
    let mut unit = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            if unit.is_empty() {
                number.push(ch);
            }
        } else if ch.is_ascii_alphabetic() {
            if !number.is_empty() {
                unit.push(ch);
            }
        } else if !number.is_empty() && !unit.is_empty() {
            break;
        }
    }

    if number.is_empty() {
        return None;
    }

    let value = number.parse::<f64>().ok()?;
    let multiplier = match unit.to_lowercase().as_str() {
        "" | "b" | "byte" | "bytes" => 1.0,
        "kb" | "kib" => 1024.0,
        "mb" | "mib" => 1024.0 * 1024.0,
        "gb" | "gib" => 1024.0 * 1024.0 * 1024.0,
        "tb" | "tib" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return None,
    };

    Some(value * multiplier)
}

fn parse_shared_flag(text: &str) -> Option<bool> {
    let normalized = text.trim().to_lowercase();
    if normalized.contains("shared") || normalized == "yes" || normalized == "true" {
        Some(true)
    } else if normalized.contains("dedicated")
        || normalized.contains("not shared")
        || normalized == "no"
        || normalized == "false"
    {
        Some(false)
    } else {
        None
    }
}

fn parse_metal_support(text: &str) -> Option<bool> {
    let normalized = text.trim().to_lowercase();
    if normalized.contains("not supported") || normalized.contains("unsupported") {
        Some(false)
    } else if normalized.contains("supported") {
        Some(true)
    } else {
        None
    }
}

fn parse_metal_feature_set(text: &str) -> Option<String> {
    let normalized = text.to_lowercase();
    let marker = "feature set";
    let index = normalized.find(marker)?;
    let start = index + marker.len();
    let feature = text[start..].trim_start_matches(&[' ', ':', ',', '-'][..]);
    if feature.is_empty() {
        None
    } else {
        Some(feature.to_string())
    }
}
