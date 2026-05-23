# FusionX-OS

FusionX-OS (TrilogyX OS) is an ARM-first operating system platform targeting Apple Silicon with a long-term goal of a unified execution environment for native Linux, containers, and virtual machines.

## Repository Structure
- `kernel/`: Modified Linux kernel patching and configuration
- `control-plane/`: Trilogy Control Plane services
- `hardware/`: Apple Silicon hardware abstraction
- `drivers/`: Device drivers
- `virtualization/`: VM orchestration and runtime integration
- `containers/`: Container runtime integration
- `filesystem/`: Unified filesystem projection
- `security/`: Identity, policy, and auditing
- `desktop/`: Desktop environment and system apps
- `tools/`: Developer tooling and CLI utilities
- `scripts/`: Build/test automation
- `tests/`: Test suites
- `docs/`: Architecture and workflow documentation

## Core Documentation
- [Ownership boundaries](docs/ownership.md)
- [Apple Silicon boot architecture](docs/architecture/apple-silicon-boot.md)
- [Kernel patch strategy](docs/architecture/kernel-patch-strategy.md)
- [Control plane architecture](docs/architecture/control-plane.md)
- [Hardware abstraction](docs/architecture/hardware-abstraction.md)
- [Telemetry and observability](docs/architecture/telemetry-observability.md)
- [Security architecture](docs/architecture/security-architecture.md)
- [Filesystem projection](docs/architecture/filesystem-projection.md)
- [Cross-layer interfaces](docs/architecture/interfaces.md)
- [Control plane MVP scope](docs/control-plane/mvp-scope.md)
- [VM/container orchestration](docs/virtualization/orchestration.md)
- [Development workflows](docs/development/workflows.md)
- [Roadmap](ROADMAP.md)
