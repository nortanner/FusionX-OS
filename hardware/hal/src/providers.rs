use crate::models::{CapabilitySnapshot, CapabilityValue};
use crate::{Capability, CapabilityClass};

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
        let mut cpu = Capability::new("cpu.cluster0", CapabilityClass::Cpu, "v1");
        cpu.properties
            .insert("core_count".to_string(), CapabilityValue::Number(0.0));
        CapabilitySnapshot::new(vec![cpu])
    }
}
