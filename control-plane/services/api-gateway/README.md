# API Gateway

**Purpose:** External REST entry point for control plane APIs.

**Responsibilities**
- Route `/v1` requests to internal services.
- Enforce request/response envelopes and auth hooks.
- Emit audit and telemetry events for API calls.

**Interfaces**
- Exposes control plane APIs to clients.
- Calls environment manager, scheduler, capability registry, policy adapter, and telemetry proxy.

**Ownership boundary:** Control Plane Team
