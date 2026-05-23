# VM and Container Orchestration Workflows

## VM Workflow
1. Request VM creation from control plane.
2. Resolve hardware capabilities and allocate resources.
3. Provision disk image and network profile.
4. Launch VM via virtualization adapter.
5. Stream telemetry and enforce policy.

## Container Workflow
1. Request container creation from control plane.
2. Apply policy and resource limits.
3. Create container runtime instance.
4. Mount filesystem projections.
5. Stream telemetry and enforce policy.

## Integration Points
- Hardware capability checks from `/hardware`.
- Security policy evaluation from `/security`.
- Filesystem projection provisioning from `/filesystem`.
