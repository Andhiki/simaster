# Roadmap

The roadmap is a sequence of milestones, not a promise of dates. Each milestone
has an exit criterion so "done" is testable. A milestone may slip; the order and
the safety constraints do not.

## Guiding constraints

- Read-only by default. Writes require a separate milestone and an RFC.
- No embedded mobile client secret and no multi-user token reuse.
- No live SIMASTER calls in PR CI.
- Every endpoint claim is `unverified` until an authorized live test is
  recorded.
- A command is not "done" until its help, JSON contract, exit codes, tests,
  and docs agree.

## Milestones

### M0 — Documentation (this wave)

Deliver: README, contributing guide, governance, security policy, support
guide, Code of Conduct, changelog, architecture, authentication, API map, CLI
contract, roadmap, repository setup, ADRs, issue forms, PR template, CODEOWNERS.

**Exit criteria**

- Every required document exists and links resolve.
- Commands, flags, exit codes, and JSON examples agree across documents.
- No secrets, real student data, or unsupported live claims.
- An independent review finds no blocking issue.

### M1 — Walking skeleton and CI (this wave)

Deliver: single-binary Rust crate, `doctor` with human and JSON output,
`completions`, typed error model, config-directory discovery, integration tests,
format/lint/test/MSRV/docs/dependency CI, weekly audit.

**Exit criteria**

- `cargo fmt --all -- --check` passes.
- `cargo clippy --all-targets --all-features -- -D warnings` passes.
- `cargo test --all-features` passes.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` passes.
- `cargo +1.85.0 check --all-features` passes.
- `doctor` never contacts the network.
- CI is green on GitHub after the first push.
- No authentication or API call is implemented.

### M2 — Authentication and identity

Deliver: web session login with CAS fallback, keyring storage with a `0600`
fallback, `auth status`, `auth logout`, `whoami`, and an experimental
`--method vnext` provider using a user-supplied client.

**Exit criteria**

- `auth login` works manually with a maintainer's own account.
- Automated tests use fixtures and mocks only; PR CI stays offline.
- `auth status` reports the provider and storage without printing secrets.
- `auth logout` removes local state and reports partial revocation failures.
- `whoami --json` uses the stable envelope and never prints a token.
- `doctor` reports the active storage and warns when the file fallback is used.
- No password is accepted in argv; no token or cookie appears in logs.
- An independent security review covers the auth diff.

### M3 — Read commands

Deliver: `profile show`, `schedule today|week`, `grades list`,
`attendance summary`, the vNext read adapter, sanitized fixtures, contract
tests, and a scheduled read-only canary.

**Exit criteria**

- Each command has a human renderer and a `--json` envelope.
- Each adapter has a fixture and a contract test; no production call in PR CI.
- A missing field or changed status returns `UPSTREAM_CHANGED`, not partial
  data.
- A read-only canary runs with a maintainer's own account and is not a required
  PR check.
- `docs/API-MAP.md` marks every endpoint with provenance, stability, and
  live-tested status.
- Timeout, rate-limit, auth-expiry, and permission paths are tested.

### M4 — Agent surface

Deliver: `--format json|jsonl`, `--no-input`, a read-only `api` escape hatch,
a separate read-only MCP server, and agent-oriented examples.

**Exit criteria**

- An agent can run every read command with `--json` and never sees a prompt.
- JSONL lists stream one object per line and remain valid on interruption.
- The MCP server reuses the same library, envelope, and error codes.
- The MCP server is read-only and cannot be enabled into write mode.
- Tool descriptions and examples do not teach an agent to print credentials or
  personal data.
- A security review covers the MCP boundary and its sandbox assumptions.

### M5 — Write operations

Deliver: an approved RFC for each write surface, `--allow-write`, `--dry-run`,
`--yes`, confirmation, idempotency, and an audit trail.

**Exit criteria**

- Every write command is off by default and refuses to run without
  `--allow-write`.
- `--dry-run` validates and prints the request without sending it.
- A destructive or state-changing action requires confirmation unless `--yes`
  is supplied.
- Retries are idempotent or explicitly rejected; replay behavior is documented.
- Audit output redacts personal data and secrets.
- Two approvals, including the security maintainer, cover the auth and write
  diff.
- The responsible-use policy and the API map are updated.

### M6 — Release and partnership

Deliver: release-plz automation, signed tags, crates.io and package-manager
artifacts, a documentation site, and engagement with UGM for a supported client
or API.

**Exit criteria**

- A published artifact installs on Linux, macOS, and Windows and reports the
  tagged version.
- The release is signed and the changelog matches the tag.
- At least two repository administrators and a security maintainer are active.
- The repository settings runbook is applied by the owner.
- A UGM contact or a documented authorization path exists for the supported
  auth method.
- A post-release security review and a rollback plan are recorded.

## Current status

| Milestone | Status | Evidence |
|---|---|---|
| M0 — Documentation | Complete locally | Docs, ADRs, templates, and link check in this branch. |
| M1 — Skeleton and CI | Complete locally; GitHub CI pending first push | Local fmt, clippy, test, doc, MSRV, and secret checks pass. |
| M2 — Auth and identity | Planned | None. |
| M3 — Read commands | Planned | None. |
| M4 — Agent surface | Planned | None. |
| M5 — Write operations | Planned | None. |
| M6 — Release and partnership | Planned | None. |

## Available now

```bash
simaster --help
simaster --version
simaster doctor
simaster doctor --json
simaster completions bash
```

`auth`, `profile`, `schedule`, `grades`, `attendance`, and `api` are registered
or documented as planned; the auth and profile commands currently return
`not implemented` with exit code 1.

## Non-goals

- A web UI, mobile app, or browser extension.
- A general SIMASTER administration tool for staff.
- Credential sharing, token pooling, or proxy accounts.
- CAPTCHA-solving services, MFA bypass, rate-limit evasion, or WAF evasion.
- Scraping other people's data or enumerating accounts.
- Write operations before M5, or any write that is not explicitly opt-in.
- Shipping the official mobile app's client ID or embedded client secret.
- A claim of UGM affiliation or endorsement.
- A guaranteed upstream compatibility promise for an undocumented API.

## Risks and dependencies

| Risk | Impact | Response |
|---|---|---|
| UGM changes the web login or CAS flow. | Auth breaks. | Typed `UPSTREAM_CHANGED`, fixture update, browser SSO target. |
| UGM requires MFA for all students. | Password flow cannot complete non-interactively. | Browser SSO with PKCE becomes the priority. |
| OAuth client registration is unavailable. | Browser SSO target stays blocked. | Keep vNext experimental with a user-supplied client; document the block. |
| The gateway envelope is not reproducible. | Read adapters cannot be built. | Do not implement; require a live capture with the user's own account. |
| The repo stays under a personal account with one admin. | Bus factor and settings risk. | Owner applies `docs/REPO-SETUP.md`; migration to an org is an M6 criterion. |
| A dependency raises the MSRV above 1.85. | CI MSRV job fails. | Raise the MSRV to the lowest working version and update Cargo.toml, rust-toolchain docs, and the workflow together. |

## How to propose a change

Open an issue with the `type:feature` label. A change to authentication, a write
surface, a breaking CLI change, or governance needs an RFC and a 7-day comment
window. See [`GOVERNANCE.md`](../GOVERNANCE.md).
