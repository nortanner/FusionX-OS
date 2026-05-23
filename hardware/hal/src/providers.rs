use crate::models::{CapabilitySnapshot, CapabilityValue};
use crate::{Capability, CapabilityClass, HealthStatus};
#[cfg(target_os = "macos")]
use std::process::Command;
use std::time::SystemTime;

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

        CapabilitySnapshot {
            generated_at: SystemTime::now(),
            capabilities,
        }
    }
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
