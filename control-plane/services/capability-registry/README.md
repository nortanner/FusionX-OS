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
