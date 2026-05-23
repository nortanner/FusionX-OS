# Ownership Boundaries

Subsystems are owned by role-based teams to keep responsibilities clear and modular.

| Directory | Ownership Boundary | Primary Responsibilities |
| --- | --- | --- |
| `/kernel` | Kernel/Core Team | Kernel patching, ARM64 configs, upstream sync |
| `/drivers` | Kernel/Drivers Team | Device drivers and low-level enablement |
| `/hardware` | Hardware Integration Team | Hardware abstraction and Apple Silicon integration |
| `/control-plane` | Control Plane Team | Orchestration, scheduling, lifecycle management |
| `/virtualization` | Virtualization Team | VM management, hypervisor integration |
| `/containers` | Containers Team | Container runtime and lifecycle management |
| `/filesystem` | Filesystem Team | Filesystem projection and storage policy |
| `/security` | Security Team | Identity, policy, auditing, isolation |
| `/desktop` | Desktop Experience Team | UX, desktop environment, system apps |
| `/tools` | Developer Experience Team | CLI tools, diagnostics, developer utilities |
| `/scripts` | Developer Experience Team | Build/test automation scripts |
| `/tests` | QA & Validation Team | Testing infrastructure and suites |

Ownership boundaries define responsibility for changes, design reviews, and interface contracts.
