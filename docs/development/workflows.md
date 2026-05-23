# Development Workflows

## Repository Conventions
- Keep subsystem changes scoped to the owning directory.
- Record architecture decisions in `docs/architecture`.
- Use clear, descriptive commit messages.

## Build/Test/Lint
- Build, test, and lint scripts will live in `/scripts`.
- Until automation is implemented, document manual steps in PR descriptions.
- When adding scripts, prefer consistent entry points: `./scripts/build`, `./scripts/test`, `./scripts/lint`.

## Documentation Conventions
- Architecture docs should include: Purpose, Goals, Interfaces, and Ownership impacts.
- Keep docs concise and focused on subsystem boundaries.
