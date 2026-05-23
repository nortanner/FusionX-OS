# Control Plane Service Scaffolding

## Service Set
- **API Gateway**: External REST entry point and routing.
- **Environment Manager**: Lifecycle orchestration for native/container/VM workloads.
- **Scheduler**: CPU/GPU/IO allocation and placement decisions.
- **Capability Registry**: Hardware and platform capability catalog.
- **Policy Adapter**: Authorization checks and audit hooks.
- **Telemetry Proxy**: Metrics/logs fan-out to observability pipeline.

## Proposed Layout
```
control-plane/
  services/
    api-gateway/
    environment-manager/
    scheduler/
    capability-registry/
    policy-adapter/
    telemetry-proxy/
  pkg/
  config/
  deployments/
```

## Startup Sequence (M1)
1. Capability registry ingests hardware capability snapshots.
2. Scheduler builds the resource model from registry state.
3. Environment manager registers runtime adapters.
4. API gateway exposes the `/v1` endpoints.

## M1 Deliverables
- Service stubs with health checks and structured logging hooks.
- Shared schema module for capability and environment types.
