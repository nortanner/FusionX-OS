# Trilogy Control Plane Architecture

## Purpose
Provide unified orchestration across native Linux, containers, and virtual machines.

## Core Services
- **Scheduler**: CPU/GPU/IO allocation across environments.
- **Environment Manager**: Lifecycle control for native, container, and VM workloads.
- **Security & Identity**: Policy enforcement, access control, audit events.
- **Filesystem Projection**: Shared storage views across execution layers.
- **Telemetry Pipeline**: Metrics, logs, and traces routed to observability tools.

## Control Plane Interfaces
- **Hardware Capability API** from `/hardware`
- **Runtime Control APIs** to `/containers` and `/virtualization`
- **Policy & Identity APIs** to `/security`

## Design Principles
- Keep orchestration logic in userspace services.
- Prefer modular, replaceable service boundaries.
- Ensure every orchestration action is observable.
