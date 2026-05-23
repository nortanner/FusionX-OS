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

## Run (M1)
- `cargo run` starts the service on `127.0.0.1:8080`.
- Set `API_GATEWAY_ADDR` to override the bind address.
- Set `CAPABILITY_REGISTRY_URL` to point at the capability registry service.

## API (M1)
- `GET /v1/health`
- `GET /v1/capabilities`
- `GET /v1/capabilities/{capabilityId}`
