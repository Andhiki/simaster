# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project documentation: README, contributing guide, governance, security
  policy, support guide, Code of Conduct, and architecture, authentication, API,
  CLI-contract, roadmap, and repository-setup documents.
- Official UGM ID OAuth 2.0/OIDC reference with endpoints, scopes, PKCE flow,
  ID-token validation, client-registration checklist, and ready-to-send DTI
  emails in English and Indonesian.
- Architecture decision records for the single-crate layout and the
  authentication strategy.
- Rust walking skeleton: `simaster doctor` with human and JSON output,
  `simaster completions`, a typed error model, config-directory discovery, and
  an offline JSON output helper.
- CLI integration tests for help output, doctor JSON, not-implemented
  authentication, and bash completions.
- CI gates for formatting, clippy, tests on Linux/macOS/Windows, MSRV 1.85,
  cargo-deny, and rustdoc. A weekly RustSec audit workflow.
- Repository metadata templates: issue forms, pull-request template,
  CODEOWNERS, Dependabot configuration, and the repository settings runbook.
- Dual licensing under MIT OR Apache-2.0.

### Changed

- Primary authentication is now the official UGM ID browser OAuth 2.0/OIDC
  Authorization Code flow with PKCE S256. The web session + CAS flow is the
  fallback when no client is registered.
- vNext is a read-only data adapter, not an authentication provider.
- `whoami` and `profile show` move to M2; `whoami` uses `/api/me` and
  `profile show` uses `/user/me`.
- `auth login --method` now accepts `auto`, `browser`, `web`, and `sso`;
  `vnext` is removed as an auth method.

### Fixed

- Initial bootstrap; no fixes yet.

### Security

- Documented the credential-handling policy: no secrets in logs, keyring-first
  storage, `0600` file fallback, redaction, and no multi-user token reuse.
- Documented that the official mobile app's embedded client secret and client
  ID are never copied, bundled, or published as usable credentials.
- Documented PKCE S256, `state`, `nonce`, loopback isolation, one-shot listener,
  and ID-token validation for the UGM ID browser flow.
- Documented the public-client limitation: a public client cannot call
  `/oauth/introspect` or `/oauth/revoke`; logout deletes local state and opens
  the browser logout endpoint when an ID token is available.
- Documented that official UGM ID endpoint behavior is `documented by UGM` and
  `live-tested: no` except the public health check; vNext and web claims remain
  `unverified`.

[Unreleased]: https://github.com/Andhiki/simaster/commits/main
