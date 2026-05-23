# Policy Adapter

**Purpose:** Policy evaluation, authorization checks, and audit integration.

**Responsibilities**
- Authorize lifecycle and scheduling actions.
- Emit audit events for policy decisions.
- Provide decision context to calling services.

**Interfaces**
- Called by the API gateway and environment manager.
- Integrates with `/security` policy services.

**Ownership boundary:** Control Plane Team
