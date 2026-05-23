# Telemetry Proxy

**Purpose:** Central ingestion point for control plane telemetry.

**Responsibilities**
- Accept metrics, logs, and trace events from services.
- Enrich telemetry with environment and service metadata.
- Forward events to the observability pipeline.

**Interfaces**
- Used by all control plane services.
- Integrates with `/security` and observability tooling.

**Ownership boundary:** Control Plane Team
