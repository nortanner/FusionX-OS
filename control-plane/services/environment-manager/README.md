# Environment Manager

**Purpose:** Lifecycle orchestration for native, container, and VM environments.

**Responsibilities**
- Create, start, stop, and restart environments.
- Track environment state and runtime assignments.
- Coordinate scheduling, policy, and telemetry hooks.

**Interfaces**
- Called by the API gateway.
- Uses the scheduler and capability registry.
- Invokes runtime adapters in `/containers` and `/virtualization`.

**Ownership boundary:** Control Plane Team

## Run (M1)
- `cargo run` starts the service on `127.0.0.1:8082`.
- Set `ENVIRONMENT_MANAGER_ADDR` to override the bind address.

## API (M1)
- `GET /health`
