# Capability Registry

## Purpose
Maintain a centralized catalog of hardware and platform capabilities for the scheduler and environment manager.

## Capability Record
- `id`: Stable identifier.
- `class`: cpu | gpu | io | memory | network | storage | accelerator.
- `vendor` / `model` / `revision`.
- `version`: Schema version for compatibility tracking.
- `location`: Physical or logical placement (socket, NUMA, VM host).
- `limits`: Capacity and allocation constraints.
- `properties`: Key/value metadata (driver, firmware, feature flags).
- `health`: ok | degraded | unavailable.
- `labels`: Scheduling and policy tags.

## Operations
- Register capability snapshots from `/hardware`.
- Update health and capacity deltas.
- Query by class, label, or location.
- Deprecate or remove stale capabilities.

## Sources of Truth
- **Hardware abstraction** publishes baseline capability snapshots and health.
- **Runtime adapters** publish runtime-specific features (hypervisor/container).

## M1 Scope
- Define schema ownership and versioning expectations.
- Document update cadence and lifecycle states.
