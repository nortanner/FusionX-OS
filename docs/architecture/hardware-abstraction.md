# Hardware Abstraction Architecture

## Goals
- Isolate Apple Silicon specifics behind stable capability interfaces.
- Allow control plane to reason about hardware without platform-specific logic.

## Capability Model
- **Compute**: CPU topology, performance tiers, power states.
- **Graphics**: GPU queues, memory bandwidth, acceleration capabilities.
- **Memory**: Unified memory properties and limits.
- **Devices**: Storage, network, and peripheral profiles.

## Contracts
- Provide a hardware capability registry consumed by the control plane.
- Use well-defined schemas for discovery, updates, and health reporting.
- Expose lifecycle events for device hotplug and power transitions.
