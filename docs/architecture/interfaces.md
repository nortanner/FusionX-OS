# Cross-Layer Interfaces and Contracts

## Control Plane ⇄ Hardware
- Capability discovery, health status, and resource allocation events.
- Contract: versioned capability schema with backward compatibility.

## Control Plane ⇄ Containers
- Lifecycle management (create/start/stop), resource limits, telemetry hooks.
- Contract: container runtime adapter interface.

## Control Plane ⇄ Virtual Machines
- VM lifecycle operations, device assignment, snapshot and recovery flows.
- Contract: hypervisor adapter interface with capability negotiation.

## Control Plane ⇄ Security
- Policy evaluation, identity verification, audit logging.
- Contract: centralized policy decision API with clear error semantics.

## Kernel ⇄ Hardware/Drivers
- Device tree and capability metadata ingestion.
- Contract: hardware abstraction ownership of device metadata formats.
