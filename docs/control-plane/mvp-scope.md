# Trilogy Control Plane MVP Scope

## MVP Goals
- Orchestrate native, container, and VM lifecycles with a single control API.
- Enforce basic security policies and collect telemetry for every action.

## MVP Deliverables
1. **Core API Service**: Lifecycle and resource allocation endpoints.
2. **Scheduler**: Basic CPU/GPU allocation model.
3. **Runtime Adapters**: Container and VM adapter stubs.
4. **Telemetry Pipeline**: Metrics/logs ingestion with local storage.
5. **Policy Enforcement**: Basic RBAC and audit logging.

## Milestones
- **M1: API Skeleton** — API definitions, service scaffolding, capability registry.
- **M2: Lifecycle Control** — Create/start/stop for containers and VMs.
- **M3: Resource Allocation** — Scheduler integration and hardware capability binding.
- **M4: Observability** — Metrics/logs pipeline and audit events.
