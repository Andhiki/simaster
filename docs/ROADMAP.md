# Roadmap

The roadmap is a sequence of milestones, not a promise of dates. Each milestone
has an exit criterion so "done" is testable. A milestone may slip; the order and
the safety constraints do not.

## Guiding constraints

- Official UGM ID OAuth 2.0/OIDC documentation is the primary auth evidence.
  Prior-art mobile-app analysis is secondary evidence for the vNext data adapter.
- Read-only by default. Writes require a separate milestone and an RFC.
- No embedded mobile client secret and no multi-user token reuse.
- No live authenticated SIMASTER calls in PR CI. A public `/health` check and
  mocked discovery/token/user endpoints are the test boundary.
- Every endpoint claim is marked `documented by UGM` or `unverified`, and
  `live-tested: no` unless an authorized live test is recorded.
- A command is not "done" until its help, JSON contract, exit codes, tests, and
  docs agree.

## Upstream track U0 — UGM ID client registration

This is an upstream dependency, not a release milestone. It gates the browser
PKCE path, not the web session fallback.

**Deliver**

- A DTI registration request for a public/native client.
- Contact: `dti@ugm.ac.id`; see [`UGM-ID-API.md`](UGM-ID-API.md) for the
  ready-to-send English and Indonesian emails.
- Requested profile: `token_endpoint_auth_method=none` if allowed for a new
  client; `authorization_code` + `refresh_token`; PKCE S256; loopback redirects
  `http://127.0.0.1:53682/callback` and, if allowed,
  `http://localhost:53682/callback`; scopes
  `openid profile email user.read userDetail.read`; a development client on
  `https://oauth.dev.ugm.ac.id`.
- Written answers to the DTI questions in [`UGM-ID-API.md`](UGM-ID-API.md),
  especially public-client support, loopback redirects, rate limits, token
  lifetimes, refresh rotation, and vNext token acceptance.

**Exit criteria**

- DTI registers a `client_id` and at least one loopback redirect URI.
- DTI confirms whether `none` is allowed for a new client, or requires a
  confidential client.
- The scopes are granted, or the narrower granted set is recorded.
- Rate limits and token lifetimes are documented.
- If DTI requires a confidential client, the project records the decision and
  keeps the web fallback; no secret is shipped.
- The result is recorded in [`API-MAP.md`](API-MAP.md) and
  [`REPO-SETUP.md`](REPO-SETUP.md), and the `blocked:upstream` label is removed
  or kept with a reason.

## Milestones

### M0 — Documentation (bootstrap wave)

Deliver: README, contributing guide, governance, security policy, support
guide, Code of Conduct, changelog, architecture, authentication, official UGM ID
reference, API map, CLI contract, roadmap, repository setup, ADRs, issue forms,
PR template, CODEOWNERS.

**Exit criteria**

- Every required document exists and links resolve.
- Commands, flags, exit codes, and JSON examples agree across documents.
- No secrets, real student data, or unsupported live claims.
- An independent review finds no blocking issue.

### M1 — Walking skeleton and CI (bootstrap wave)

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

### M2 — UGM ID login and profile proof of concept

Deliver: browser OAuth 2.0 Authorization Code + PKCE S256 with a public client
and loopback redirect; `/api/me`; `/user/me`; `whoami`; `profile show`;
`auth status`; `auth logout`; keyring storage with a `0600` fallback; and the
web session + CAS fallback while U0 is pending.

**Exit criteria**

- `auth login --method browser` completes manually with a DTI-registered client
  and a maintainer's own account.
- `auth login --method auto` uses browser PKCE when `SIMASTER_CLIENT_ID` is set,
  otherwise the web session + CAS fallback.
- `auth login --method browser` without a client ID fails with `CONFIG`.
- `whoami --json` calls `/api/me`; `profile show --json` calls `/user/me`; both
  use the stable envelope and never print a token.
- PKCE S256, `state`, `nonce`, loopback binding, one-shot listener, timeout, and
  browser-open fallback are covered by tests with mocked endpoints.
- ID-token validation covers signature/JWKS, `iss`, `aud`, `exp`, and `nonce`,
  including the discovery issuer scheme quirk.
- `auth status` reports provider, username, scopes, expiry, and storage without
  token values.
- `auth logout` deletes local state, opens browser logout when possible, and
  reports the public-client introspection/revocation limitation.
- No password is accepted in argv; no token, code, cookie, or verifier appears
  in logs.
- An independent security review covers the auth diff.

### M3 — Data commands

Deliver: `schedule today|week`, `grades list`, `attendance summary`, the vNext
read adapter, the web read adapter, sanitized fixtures, contract tests, and a
scheduled read-only canary. Test whether the official UGM ID access token is
accepted by `api.simaster.ugm.ac.id`; use the web adapter if it is not.

**Exit criteria**

- Each command has a human renderer and a `--json` envelope.
- Each adapter has a fixture and a contract test; no production call in PR CI.
- The official-token acceptance test is recorded with a date and a sanitized
  result. If accepted, the vNext adapter is `experimental`; if not, the web
  adapter is the documented path.
- The profile fallback works: `/user/me` primary, web scrape fallback, vNext
  `get_student_profil_by_niu` only if the official token is accepted
  (`unverified`).
- A missing field or changed status returns `UPSTREAM_CHANGED`, not partial
  data.
- A read-only canary runs with a maintainer's own account and is not a required
  PR check.
- `docs/API-MAP.md` marks every endpoint with level, provenance, stability, and
  live-tested status.
- Timeout, rate-limit, auth-expiry, and permission paths are tested.

### M4 — Agent surface

Deliver: `--format json|jsonl`, `--no-input`, a read-only `api` escape hatch,
a separate read-only MCP server, and agent-oriented examples.

**Exit criteria**

- An agent can run every read command with `--json` and never sees a prompt.
- Browser login remains explicitly human; `--no-browser` prints the URL and
  never automates consent.
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
artifacts, a documentation site, and engagement with UGM/DTI for a supported
client or API.

**Exit criteria**

- A published artifact installs on Linux, macOS, and Windows and reports the
  tagged version.
- The release is signed and the changelog matches the tag.
- At least two repository administrators and a security maintainer are active.
- The repository settings runbook is applied by the owner.
- The UGM ID client registration (U0) is resolved, or the web fallback is the
  documented supported path.
- A post-release security review and a rollback plan are recorded.

## Current status

| Track or milestone | Status | Evidence |
|---|---|---|
| M0 — Documentation | Complete locally | Docs, ADRs, templates, and link check in this branch. |
| M1 — Skeleton and CI | Complete locally; GitHub CI pending first push | Local fmt, clippy, test, doc, MSRV, actionlint, cargo-deny, and secret checks pass. |
| U0 — UGM ID client registration | Pending; request drafted | DTI contact and email are in `UGM-ID-API.md`; no client is registered. |
| M2 — UGM ID login and profile | Planned | Official API reference and auth design are in place; CLI returns not implemented. |
| M3 — Data commands | Planned | vNext data adapter is documented; token acceptance is unverified. |
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
`not implemented` with exit code 1. `whoami` is planned for M2 and is not in the
M1 command tree.

## Non-goals

- A web UI, mobile app, or browser extension.
- A general SIMASTER administration tool for staff.
- Credential sharing, token pooling, or proxy accounts.
- The mobile app's embedded client secret or custom-scheme redirect as a usable
  credential.
- A password grant, CAPTCHA-solving service, MFA bypass, rate-limit evasion, or
  WAF evasion.
- Scraping other people's data, enumerating accounts, or using
  `/user/username/{username}`.
- Write operations before M5, or any write that is not explicitly opt-in.
- A backend-for-frontend in v0. Revisit only through an RFC if DTI requires a
  confidential client and no user-supplied client is workable.
- A claim of UGM affiliation or endorsement.
- A guaranteed upstream compatibility promise for an undocumented API.

## Risks and dependencies

| Risk | Impact | Response |
|---|---|---|
| DTI has not registered a client. | Browser PKCE cannot complete. | Web session + CAS fallback remains the default; U0 tracks the request. |
| DTI requires a confidential client. | A public/native client with no secret is unavailable. | Do not ship a secret; keep web fallback, allow a user-registered confidential client, or revisit a BFF in an RFC. |
| `none` is legacy-only and may be removed. | The public-client path could stop working. | Get written DTI confirmation; keep web fallback; do not rely on a removed grant. |
| Discovery issuer and endpoints omit the scheme. | Strict ID-token validation can fail closed on `iss`. | Record the inconsistency; validate the literal issuer; normalize transport only; report `UPSTREAM_CHANGED`. |
| Live PKCE and profile behavior is untested. | The design may miss upstream details. | Mocked tests first; authorized live test with the user's own account in M2. |
| The vNext gateway rejects the official token. | The vNext data adapter is unusable. | Test acceptance in M3; use the web data adapter. Never add a mobile-client auth provider. |
| A public client cannot introspect or revoke. | Server-side logout is incomplete. | Open `/oauth/logout` with `id_token_hint`; delete local state; report the limitation. |
| UGM changes the web login or CAS flow. | The fallback breaks. | Typed `UPSTREAM_CHANGED`, fixture update, browser PKCE primary. |
| UGM requires MFA for all students. | The web fallback cannot complete non-interactively. | Browser PKCE is the primary path; the web flow is explicitly lower assurance. |
| The repo stays under a personal account with one admin. | Bus factor and settings risk. | Owner applies `docs/REPO-SETUP.md`; migration to an org is an M6 criterion. |
| A dependency raises the MSRV above 1.85. | CI MSRV job fails. | Raise the MSRV to the lowest working version and update Cargo.toml, rust-toolchain docs, and the workflow together. |

## How to propose a change

Open an issue with the `type:feature` label. A change to authentication, a write
surface, a breaking CLI change, or governance needs an RFC and a 7-day comment
window. See [`GOVERNANCE.md`](../GOVERNANCE.md).
