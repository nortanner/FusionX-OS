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
