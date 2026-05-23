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

## Transport Payload
- Hardware HAL publishes `{ generated_at, capabilities }` payloads to the capability registry transport.
- `generated_at` is reported as epoch seconds.
- GPU properties normalize Metal/VRAM fields:
  - `metal_support` (boolean), `metal_feature_set` (string), optional `metal_support_detail`.
  - `vram_bytes` (number), `vram_shared` (boolean), optional `vram_shared_bytes` or `vram_shared_detail`.

## Sources of Truth
- **Hardware abstraction** publishes baseline capability snapshots and health.
- **Runtime adapters** publish runtime-specific features (hypervisor/container).

## M1 Scope
- Define schema ownership and versioning expectations.
- Document update cadence and lifecycle states.
