# Contributing to simaster

Thanks for helping build an unofficial, terminal-first SIMASTER client. This
project values small, reviewable changes, evidence over assumptions, and safe
handling of student data.

By participating, you agree to the [Code of Conduct](CODE_OF_CONDUCT.md).

## Prerequisites

- **Rust stable** (the repository pins `stable` in `rust-toolchain.toml`).
- **MSRV 1.85** — edition 2024 requires it. The CI `msrv` job checks this.
- **`just`** (optional) — convenience recipes in the `justfile`.
- **`cargo-nextest`** (optional) — faster local test runs.
- **`cargo-deny`** and **`cargo-audit`** (optional) — dependency gates. CI runs
  them; local runs are encouraged before dependency changes.

No SIMASTER account is needed for normal development. The test suite is
offline.

## Setup

```bash
git clone https://github.com/Andhiki/simaster
cd simaster
cargo build
cargo test --all-features
```

If you use `just`:

```bash
just          # same as just check
just run -- --help
```

## Build, test, format, lint, docs

```bash
cargo build
cargo test --all-features
cargo nextest run --all-features   # optional alternative to cargo test
cargo fmt --all
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo +1.85.0 check --all-features
cargo deny check                   # optional; CI enforces
cargo audit                        # optional; CI runs weekly
```

`just check` runs format check, clippy, tests, and docs. CI runs tests on Linux,
macOS, and Windows; the format, clippy, MSRV, dependency, and docs jobs run on
Ubuntu. All jobs are required checks.

## First contribution path

1. Pick an issue labelled `good first issue`.
2. Comment that you intend to work on it and wait for a maintainer to assign it.
   This avoids duplicate work.
3. Ask questions in the issue before guessing at behavior.
4. Open a draft pull request early if you want feedback on direction.
5. Keep the first change small. Documentation, test-only, and `doctor`/
   `completions` improvements are good starting points.

If an issue is not labelled `good first issue`, comment first and agree on the
scope with a maintainer.

## Branch naming

Create a branch from `main`:

```text
feat/<issue>-slug
fix/<issue>-slug
docs/<issue>-slug
refactor/<issue>-slug
test/<issue>-slug
chore/<issue>-slug
```

Examples: `feat/42-auth-login`, `docs/17-endpoint-provenance`,
`fix/81-exit-code-mapping`. Keep one concern per branch.

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org/):

```text
type(scope): imperative summary (<= 72 characters)

What: the behavior or document that changed.
Why: the problem, issue, or upstream evidence that motivated it.
Impact: the user-visible or maintenance consequence.
Evidence: commands run, tests added, or sources checked.
Risks: known gaps, compatibility concerns, or "none identified".
```

Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `perf`, `build`,
`ci`, `chore`, `revert`.

A behavior change and its tests are one commit. Do not commit a failing test
first and the implementation second, and do not add a `test:` commit after a
`feat:` commit for the same behavior. Commit at green.

Keep commits atomic. Do not mix formatting-only churn with behavior changes.

## Tests

- **Unit tests** live next to the code and cover pure logic and error mapping.
- **Integration tests** live in `tests/` and exercise the binary with
  `assert_cmd` and `predicates`.
- **Fixtures** live under `tests/fixtures/` when they are added. They must be
  sanitized: no real names, NIU numbers, cookies, tokens, or passwords.
- **HTTP tests** use `wiremock` or another local mock once HTTP lands. Auth
  tests mock discovery, the token endpoint, JWKS, and the user endpoints. PR CI
  must never contact `simaster.ugm.ac.id`, `sso.ugm.ac.id`,
  `oauth.simaster.ugm.ac.id`, or `api.simaster.ugm.ac.id`.
- **Canary tests** are scheduled jobs that optionally use a maintainer's own
  account. They are not part of PR CI and must fail loudly with
  `UPSTREAM_CHANGED` when the upstream contract moves.

Every new command, flag, exit-code path, and JSON field needs a test. Prefer
behavior assertions over implementation details. If a test cannot be written
without a live server, use a sanitized fixture and mark the behavior
`unverified`.

Authentication changes need an independent security review before merge. The
review covers PKCE, `state`, `nonce`, loopback binding, ID-token validation,
token storage, redaction, and the public-client logout limitation.

## Endpoint pull requests

An endpoint addition or change must include:

1. An update to [`docs/API-MAP.md`](docs/API-MAP.md) with the level, surface,
   method, path, auth, request shape, response shape, provenance, stability, and
   live-tested status.
2. A sanitized fixture (request and response) with credentials, identifiers,
   and personal data removed or replaced with documented placeholders.
3. A contract test that exercises the adapter against the fixture. The test
   must not contact production.
4. Provenance: where the evidence came from and the date it was observed.
   Official UGM ID OpenAPI/Swagger, OIDC discovery, and portal documentation
   are primary evidence; see [`docs/UGM-ID-API.md`](docs/UGM-ID-API.md).
   Prior-art static analysis is secondary and must be labelled as such.
5. A stability statement: `unverified`, `experimental`, `stable`, or
   `blocked`, and what would make it change.

Do not include the mobile app's client ID, embedded client secret, tokens,
cookies, authorization codes, PKCE verifiers, the mobile custom-scheme redirect
as a usable credential, or real student data. Placeholders such as
`<client-id>` and `<niu>` are acceptable. A write endpoint needs an RFC, two
approvals, a dry-run path, and an explicit opt-in flag before it can be merged.

## Pull requests

- Link the issue with `Closes #123` or `Refs #123`.
- Keep one concern per PR. Split unrelated refactors.
- Aim for fewer than 400 changed lines where practical; explain larger PRs.
- Include command output, JSON, or a screenshot for user-visible CLI changes.
- Keep CI green. A red required check is not ready for review.
- One approval is required. Two approvals are required for authentication,
  security, release, governance, and breaking changes.
- Update docs and the changelog when behavior changes.

Use the pull request template. Fill in the testing evidence and the
secret/PII checklist honestly. Draft PRs are welcome for early feedback.

## Review expectations

Reviewers should:

- Check correctness, security, data handling, and compatibility before style.
- Ask for evidence when a claim about SIMASTER behavior is not supported.
- Keep review comments actionable and scoped to the change.
- Avoid approving code they do not understand.

Authors should respond to every blocking comment, resolve conversations, and
re-run the full verification after changes. A review is not an approval until
the reviewer says so.

**Stale policy:** a PR with no author activity for 30 days is marked stale. A
maintainer pings the author; after 60 days it may be closed. Closed PRs can be
reopened when the author returns. Issues follow the same cadence.

## Release process

Releases are prepared with [`release-plz`](https://github.com/MarcoIeni/release-plz)
and follow [Semantic Versioning](https://semver.org/). Before 1.0, minor
versions may include breaking changes, but they must be called out in the
changelog and PR.

A release requires:

1. Green CI on `main`.
2. A release-plz pull request with the version, changelog, and tag proposal.
3. Approval from the release manager and one other maintainer.
4. A signed tag and a GitHub release with the changelog.
5. Post-release verification that the published artifact installs and
   `simaster --version` matches the tag.

Security releases follow the coordinated disclosure process in
[`SECURITY.md`](SECURITY.md).

## Security

Do not report vulnerabilities in a public issue, discussion, or pull request.
Use GitHub Security Advisories as described in [`SECURITY.md`](SECURITY.md).

Never commit:

- passwords, tokens, cookies, or session files;
- real student names, NIU numbers, grades, schedules, or attendance records;
- the mobile app's embedded client credentials;
- logs that contain any of the above.

If a secret is committed by accident, treat it as compromised: remove it,
rotate it, and notify a maintainer immediately.

## AI-assisted contributions

AI-assisted contributions are allowed. You are responsible for the result:
review every line, run the tests, and understand the design. Disclose
substantial AI assistance in the PR description (which parts and which tools).
Do not paste credentials, tokens, personal data, or private upstream evidence
into a model prompt. Do not let an agent add a dependency, endpoint, or
permission without the same review a human change would receive.

## Where to go next

- [GOVERNANCE.md](GOVERNANCE.md) — roles, consensus, RFCs.
- [SECURITY.md](SECURITY.md) — vulnerability reporting and credential rules.
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) — community standards.
- [docs/ROADMAP.md](docs/ROADMAP.md) — milestones and current priorities.
- [docs/CLI-CONTRACT.md](docs/CLI-CONTRACT.md) — the command and output contract.
- [docs/API-MAP.md](docs/API-MAP.md) — endpoint evidence and stability.
