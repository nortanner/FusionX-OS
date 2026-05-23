# Unified Filesystem Projection

## Goals
- Provide consistent storage views across native, container, and VM layers.
- Support secure sharing with policy-controlled access.

## Projection Model
- **Base Layer**: Host filesystem view.
- **Environment Layer**: Per-environment overlays and isolation.
- **Shared Mounts**: Explicit shared resources with policy enforcement.

## Coordination
- Control plane owns lifecycle and mount policy.
- Security policies gate access across environments.
