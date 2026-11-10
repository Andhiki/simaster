# Contributing to simaster

Thanks for helping build an unofficial, terminal-first SIMASTER client. This
project values small, reviewable changes, evidence over assumptions, and safe
handling of student data. By participating, you agree to the
[Code of Conduct](CODE_OF_CONDUCT.md).

## Prerequisites

- **Rust stable** (pinned in `rust-toolchain.toml`); **MSRV 1.85** for edition
  2024. CI checks the MSRV.
- **`just`** (optional) — recipes in the `justfile`.
- **`cargo-nextest`**, **`cargo-deny`**, **`cargo-audit`** (optional) — faster
  tests and dependency gates. CI runs the gates; run them before dependency
  changes.

No SIMASTER account is needed. The test suite is offline.

## Setup and commands

```bash
git clone https://github.com/Andhiki/simaster
cd simaster
cargo build
```

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo +1.85.0 check --all-features
cargo deny check                   # optional; CI enforces
cargo audit                        # optional; CI runs weekly
```

`just check` runs format check, clippy, tests, and docs. CI runs tests on Linux,
macOS, and Windows; format, clippy, MSRV, dependency, and docs jobs run on
Ubuntu. All jobs are required checks.

## First contribution path

1. Pick an issue labelled `good first issue` and comment that you intend to work
   on it. Wait for a maintainer to assign it; this avoids duplicate work.
2. Ask questions in the issue before guessing at behavior.
3. Open a draft pull request early for direction feedback.
4. Keep the first change small. Documentation, test-only, and `doctor`/
   `completions` improvements are good starting points.

If an issue is not labelled `good first issue`, agree on the scope first. Issues
live in [GitHub Issues](https://github.com/pendopo-dev/simaster/issues).

## Branch naming

Create a branch from `main` with one concern: `feat/<issue>-slug`,
`fix/<issue>-slug`, `docs/<issue>-slug`, `refactor/<issue>-slug`,
`test/<issue>-slug`, or `chore/<issue>-slug`. Examples: `feat/42-auth-login`,
`docs/17-endpoint-provenance`.

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org/):

```text
type(scope): imperative summary (<= 72 characters)

What: the behavior or document that changed.
Why: the problem, issue, or upstream evidence.
Impact: the user-visible or maintenance consequence.
Evidence: commands run, tests added, or sources checked.
Risks: known gaps, compatibility concerns, or "none identified".
```

Allowed types: `feat`, `fix`, `refactor`, `test`, `docs`, `perf`, `build`,
`ci`, `chore`, `revert`.

A behavior change and its tests are one commit. Do not commit a failing test
first and the implementation second, and do not add a `test:` commit after a
`feat:` commit for the same behavior. Commit at green and keep commits atomic.

## Tests

- **Unit tests** live next to the code and cover pure logic and error mapping.
- **Integration tests** live in `tests/` and use `assert_cmd` and `predicates`.
- **Fixtures** live under `tests/fixtures/` and must be sanitized: no real
  names, NIU numbers, cookies, tokens, or passwords.
- **HTTP tests** use `wiremock` or another local mock. Auth tests mock discovery,
  the token endpoint, JWKS, and user endpoints. PR CI must never contact
  `simaster.ugm.ac.id`, `sso.ugm.ac.id`, `oauth.simaster.ugm.ac.id`, or
  `api.simaster.ugm.ac.id`.
- **Canary tests** are scheduled jobs that may use a maintainer's own account.
  They are not PR checks and fail loudly with `UPSTREAM_CHANGED` on drift.

Every new command, flag, exit-code path, and JSON field needs a test. Prefer
behavior assertions over implementation details. If a test needs a live server,
use a sanitized fixture and mark the behavior `unverified`. Authentication
changes need an independent security review covering PKCE, `state`, `nonce`,
loopback binding, ID-token validation, storage, redaction, and logout limits.

## Endpoint changes

An endpoint addition or change needs a sanitized fixture, a contract test that
does not contact production, and provenance: where the evidence came from and
the date observed. Official UGM ID OpenAPI/Swagger, OIDC discovery, and portal
documentation are primary; prior-art static analysis is secondary and must be
labelled. Record a stability value (`unverified`, `experimental`, `stable`, or
`blocked`) and what would change it.

Update [`docs/AUTH.md`](docs/AUTH.md) for authentication endpoints, or the
relevant GitHub issue for data-adapter evidence. Never include the mobile app's
client ID, embedded secret, tokens, cookies, authorization codes, PKCE
verifiers, the custom-scheme redirect as a usable credential, or real student
data. A write endpoint needs an RFC, two approvals, a dry-run path, and an
explicit opt-in flag.

## Pull requests

- Link the issue with `Closes #123` or `Refs #123`; keep one concern per PR.
- Aim for fewer than 400 changed lines; explain larger PRs.
- Include command output, JSON, or a screenshot for user-visible changes.
- Keep CI green. A red required check is not ready for review.
- One approval is required; two for authentication, security, release,
  governance, and breaking changes.
- Update the docs when behavior changes. There is no changelog file.

Use the pull request template and fill in testing evidence and the secret/PII
checklist honestly. Draft PRs are welcome.

## Review expectations

Review correctness, security, data handling, and compatibility before style; ask for evidence when a claim is unsupported; avoid approving code you do not understand. Authors respond to every blocking comment and re-run verification. A PR with no author activity for 30 days is stale; after 60 days it may be closed.

## Documentation

- Repository docs describe current state only. No plans, milestones, or delivery history.
- Task state lives in GitHub Issues; scratchpads live in the gitignored `docs/plans/`.
- A behavior change updates the command table in [`README.md`](README.md) and [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md); auth and architecture changes update [`docs/AUTH.md`](docs/AUTH.md) and [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md).
- Do not add a changelog or worker report to the repository.

## Release process

Releases follow [Semantic Versioning](https://semver.org/) and use [`release-plz`](https://github.com/MarcoIeni/release-plz). Before 1.0, minor versions may include breaking changes; call them out in the pull request and release notes. Automation is tracked in [#46](https://github.com/pendopo-dev/simaster/issues/46).

## Security

Do not report vulnerabilities in a public issue, discussion, or PR; use GitHub Security Advisories as described in [`SECURITY.md`](SECURITY.md). Never commit passwords, tokens, cookies, session files, real student data, the mobile app's embedded credentials, or logs containing any of them. If a secret is committed by accident, treat it as compromised: remove it, rotate it, and notify a maintainer.

## AI-assisted contributions

AI-assisted contributions are allowed, and you are responsible for the result: review every line, run the tests, and understand the design. Disclose substantial assistance in the PR description. Do not paste credentials, tokens, personal data, or private upstream evidence into a prompt. Do not let an agent add a dependency, endpoint, or permission without human-equivalent review.
