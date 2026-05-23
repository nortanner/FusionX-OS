# Kernel Patch Strategy

## Baseline
- Track an ARM64-capable Linux LTS kernel as the primary baseline.
- Maintain a clean separation between upstream patches and local enablement.

## Patch Categories
1. **Apple Silicon Enablement**: SoC bring-up, device drivers, boot adjustments.
2. **Virtualization Enhancements**: KVM/QEMU improvements required by the VM layer.
3. **Scheduler Experiments**: Optional patches behind feature flags.
4. **Security Hardening**: Mandatory kernel security improvements.

## Patch Management
- Keep patches in a dedicated queue with clear ownership and review.
- Upstream whenever feasible; track upstream status in changelogs.
- Avoid hard-coding platform assumptions; prefer capability checks.

## Release Cadence
- Sync with upstream LTS cadence.
- Publish a patch manifest and compatibility matrix per release.

## Validation
- Require boot sanity on Apple Silicon.
- Require regression checks for VM/container features.
