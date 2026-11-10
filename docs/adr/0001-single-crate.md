# ADR 0001: Start with a single crate

- **Status:** Accepted
- **Date:** 2026-11-10
- **Deciders:** project maintainers

## Context

The project is starting from an empty repository. It needs a compilable CLI,
a library core for tests and a future MCP server, and a dependency set that is
small enough to audit. The first milestone has no authentication or HTTP code,
so there is no second consumer of an auth crate yet.

A workspace with several crates would add path dependencies, version
coordination, and release overhead before there is a boundary that needs them.
A single crate keeps the build fast and the review surface small. The binary
and library targets can still be separated by module boundaries.

## Decision

Use one package, `simaster-cli`, with:

- a binary target named `simaster` at `src/main.rs`;
- a library target at `src/lib.rs` with the reusable core;
- modules `cli`, `config`, `error`, and `output` now;
- `auth` and `api` modules added inside the same crate in later milestones.

The binary stays thin: it calls `simaster_cli::run()`, prints a redacted error,
and returns the exit code. Logic belongs in the library so integration tests,
the future MCP server, and any future front end can reuse it.

## Consequences

Positive:

- One `Cargo.toml`, one lockfile, one version, one release.
- Faster clean builds and less CI matrix overhead.
- The library boundary is available for the MCP server without a workspace.
- Dependency review is concentrated in one place.

Negative:

- Module discipline must be enforced by review; the compiler cannot prevent a
  command module from importing an adapter directly.
- A future split will require moving files and updating imports.
- Feature flags for experimental surfaces must be managed within one crate.

Mitigations:

- The dependency direction is documented in `docs/ARCHITECTURE.md`:
  `cli -> auth/api -> output/config/error`.
- `auth` and `api` must not depend on `cli`.
- The MCP server, when added, is a separate binary in the same package or a
  separate package that depends on the library, not a copy of the logic.

## Split criteria

Split the crate when one of these is true:

1. The MCP server needs a dependency or runtime that the CLI should not carry
   (for example, an async runtime or a transport stack).
2. The auth or API layer has a second independent consumer and its public API
   is stable enough to version separately.
3. Build time or binary size makes a single artifact impractical.
4. A security boundary requires a separate process or permission model.
5. Release cadences genuinely diverge and the single version becomes a
   constraint.

A split is an RFC under `GOVERNANCE.md`. The RFC states the new crate graph, the
migration plan, and how the JSON contract and tests remain shared.

## Alternatives considered

- **Workspace from day one:** rejected because it adds coordination cost before
  a real boundary exists.
- **Binary only, no library:** rejected because the MCP server and integration
  tests need reusable logic, and `assert_cmd` tests should exercise the same
  core the binary uses.
- **Separate `simaster-core` and `simaster-cli` crates now:** rejected for the
  same reason as the workspace; revisit when the split criteria are met.
