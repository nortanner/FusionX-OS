use std::collections::BTreeMap;
use std::time::SystemTime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Ok,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapabilityClass {
    Cpu,
    Gpu,
    Io,
    Memory,
    Network,
    Storage,
    Accelerator,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CapabilityValue {
    Bool(bool),
    Number(f64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Capability {
    pub id: String,
    pub class: CapabilityClass,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub revision: Option<String>,
    pub version: String,
    pub location: Option<String>,
    pub limits: BTreeMap<String, CapabilityValue>,
    pub properties: BTreeMap<String, CapabilityValue>,
    pub health: HealthStatus,
    pub labels: Vec<String>,
}

impl Capability {
    pub fn new(id: impl Into<String>, class: CapabilityClass, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            class,
            vendor: None,
            model: None,
            revision: None,
            version: version.into(),
            location: None,
            limits: BTreeMap::new(),
            properties: BTreeMap::new(),
            health: HealthStatus::Ok,
            labels: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilitySnapshot {
    pub generated_at: SystemTime,
    pub capabilities: Vec<Capability>,
}

impl CapabilitySnapshot {
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self {
            generated_at: SystemTime::now(),
            capabilities,
        }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
}
