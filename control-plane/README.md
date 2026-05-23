# Control Plane

**Purpose:** Trilogy Control Plane services for orchestration and lifecycle management.

**Responsibilities**
- Resource scheduling and environment lifecycle coordination
- Identity, policy, and security coordination
- Cross-environment filesystem projection and telemetry hooks

**Interfaces**
- Manages `/virtualization` and `/containers`
- Integrates hardware allocation signals from `/hardware`

**Ownership boundary:** Control Plane Team

## Docs
- [MVP scope](../docs/control-plane/mvp-scope.md)
- [API skeleton](../docs/control-plane/api.md)
- [Service scaffolding](../docs/control-plane/service-scaffolding.md)
- [Capability registry](../docs/control-plane/capability-registry.md)
