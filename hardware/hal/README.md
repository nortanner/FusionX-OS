# Hardware HAL

**Purpose:** Rust hardware abstraction layer for Apple Silicon capability discovery.

**Responsibilities**
- Provide capability snapshots for CPU/GPU/memory/device inventory.
- Expose health and metadata needed by the control plane capability registry.
- Define provider interfaces for platform-specific discovery.

**Interfaces**
- Publishes capability snapshots to `/control-plane` services.
- Consumes platform data from kernel and device drivers.

**Ownership boundary:** Hardware Integration Team
