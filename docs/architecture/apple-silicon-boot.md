# Apple Silicon Boot Architecture

## Goals
- Boot reliably on Apple Silicon hardware while remaining portable to other ARM64 devices.
- Keep platform-specific logic isolated behind a hardware abstraction layer.
- Preserve a clean kernel/userspace boundary.

## Boot Flow (High-Level)
1. **Platform Boot ROM / Secure Boot** initializes the device and verifies next-stage boot.
2. **Apple Boot Chain** hands off to an open boot loader stage (e.g., m1n1 or equivalent).
3. **Boot Loader Stage** prepares memory, loads the kernel, and hands off device metadata.
4. **Kernel Early Init** loads ARM64 kernel image and applies Apple Silicon enablement.
5. **User Space Init** starts the control plane, logging pipeline, and system services.

## Responsibilities by Layer
- **Boot Loader**: Hardware discovery, device tree/metadata handoff, early console.
- **Kernel**: SoC bring-up, MMU/interrupts, storage, networking, GPU device enablement.
- **Control Plane**: Resource orchestration and environment lifecycle after kernel init.

## Required Artifacts
- Boot loader configuration and device metadata contract.
- Kernel configuration profile for Apple Silicon.
- Minimal init sequence for control plane bootstrap.

## Portability Considerations
- Keep device-specific logic in `/hardware` or `/drivers`.
- Use a common boot metadata schema for all ARM64 devices.
