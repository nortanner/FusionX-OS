# Scheduler

**Purpose:** Resource allocation and placement decisions across environments.

**Responsibilities**
- Build a resource model from capability registry data.
- Select placement for workloads based on requested resources.
- Track allocations and release resources on teardown.

**Interfaces**
- Consumes capability registry updates.
- Invoked by the environment manager.

**Ownership boundary:** Control Plane Team
