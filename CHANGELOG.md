# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Project documentation: README, contributing guide, governance, security
  policy, support guide, Code of Conduct, and architecture, authentication, API,
  CLI-contract, roadmap, and repository-setup documents.
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

- Initial bootstrap; no user-visible changes yet.

### Fixed

- Initial bootstrap; no fixes yet.

### Security

- Documented the credential-handling policy: no secrets in logs, keyring-first
  storage, `0600` file fallback, redaction, and no multi-user token reuse.
- Documented that the official mobile app's embedded client secret and client
  ID are never copied, bundled, or published as usable credentials.
- Documented that all endpoint behavior is `unverified` until an authorized
  live test is recorded.

[Unreleased]: https://github.com/Andhiki/simaster/commits/main
