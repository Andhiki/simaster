# ADR 0001: Start with a single crate

- **Status:** Accepted
- **Date:** 2026-11-10
- **Deciders:** project maintainers

## Context

The repository needs a compilable CLI, a reusable library core for tests and a
future MCP server, and a dependency set small enough to audit. A workspace with
several crates would add path dependencies, version coordination, and release
overhead before a boundary needs them. A single crate keeps builds fast and the
review surface small; module boundaries still separate the targets.

## Decision

Use one package, `simaster-cli`, with:

- a binary target named `simaster` at `src/main.rs`;
- a library target at `src/lib.rs` with the reusable core;
- modules `cli`, `config`, `error`, and `output`; auth and API modules live in
  the same crate when implemented.

The binary stays thin: it calls `simaster_cli::run_with(Cli::parse())`, prints a
redacted error, and returns the exit code. Logic belongs in the library so
integration tests, the MCP server, and any future front end reuse it.

## Consequences

One `Cargo.toml`, lockfile, version, and release; faster clean builds and less
CI overhead; the library boundary is available without a workspace. Module
discipline must be enforced by review, a future split moves files, and
experimental surfaces share one feature-flag space.

Mitigations: the dependency direction is `cli -> auth/api -> output/config/error`;
`auth` and `api` must not depend on `cli`; the MCP server is a separate binary
or package that depends on the library, not a copy of the logic.

## Split criteria

Split the crate when the MCP server needs a dependency or runtime the CLI should
not carry, the auth or API layer has a second independent consumer with a stable
API, build time or binary size makes one artifact impractical, a security
boundary needs a separate process, or release cadences genuinely diverge. A
split is an RFC under `GOVERNANCE.md`.

## Alternatives considered

- **Workspace from day one:** rejected; coordination cost before a real boundary.
- **Binary only:** rejected; the MCP server and integration tests need reusable
  logic.
- **Separate core and CLI crates now:** rejected for the same reason; revisit
  when the split criteria are met.
