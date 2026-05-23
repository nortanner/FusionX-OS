# Capability Registry

**Purpose:** Central catalog of hardware and platform capabilities.

**Responsibilities**
- Ingest capability snapshots and health updates from `/hardware`.
- Provide queryable capability views for the scheduler.
- Track capability lifecycle and availability changes.

**Interfaces**
- Receives capability updates from `/hardware`.
- Serves the scheduler and environment manager.

**Ownership boundary:** Control Plane Team

## Run (M1)
- `cargo run` starts the service on `127.0.0.1:8081`.
- Set `CAPABILITY_REGISTRY_ADDR` to override the bind address.

## API (M1)
- `GET /health` — service health.
- `GET /v1/capabilities` — list registered capabilities.
- `GET /v1/capabilities/{capabilityId}` — fetch a single capability.
- `POST /v1/capabilities/register` — register a capability snapshot payload.
