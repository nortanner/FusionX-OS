# Control Plane API Skeleton

## Goals
- Provide versioned lifecycle and resource APIs for native, container, and VM workloads.
- Establish consistent envelopes and error semantics for M1 scaffolding.

## Versioning and Base Path
- REST/JSON over HTTPS.
- Base path: `/v1`.
- Response envelope: `{ requestId, status, data, error }`.

## Core Resources
- **Environments**: Workloads across native, container, and VM runtimes.
- **Runtimes**: Container and VM adapters with capability negotiation.
- **Capabilities**: Hardware and platform capability inventory.
- **Policies**: Authorization decisions (placeholder for M2/M4).
- **Telemetry**: Audit and metric emission (placeholder for M4).

## Endpoint Skeleton
### Health
- `GET /v1/health`
  - Provided by the API gateway with `{ requestId, status, data, error }` envelope.

### Capabilities
- `GET /v1/capabilities`
- `GET /v1/capabilities/{capabilityId}`
- `POST /v1/capabilities/register` (capability registry ingestion)
  - API gateway proxies `GET` requests to the capability registry service.

### Environments
- `POST /v1/environments`
- `GET /v1/environments`
- `GET /v1/environments/{environmentId}`
- `DELETE /v1/environments/{environmentId}`
- `POST /v1/environments/{environmentId}/actions` (start|stop|restart)

### Runtimes
- `GET /v1/runtimes`
- `GET /v1/runtimes/{runtimeId}`
