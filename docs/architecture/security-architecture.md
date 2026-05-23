# Security Architecture

## Principles
- Security-first design with minimal trust boundaries.
- Least-privilege access across all execution environments.

## Core Components
- **Identity & Access**: User and service identities, RBAC policies.
- **Policy Engine**: Enforces container/VM constraints and runtime rules.
- **Audit**: Immutable event log for control plane actions.
- **Secure Boot**: Verify boot chain integrity and kernel authenticity.

## Integration Points
- Control plane enforces policies across all execution layers.
- Kernel and drivers expose security-relevant telemetry to `/security`.
