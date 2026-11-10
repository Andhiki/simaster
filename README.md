# simaster

Unofficial terminal-first CLI for SIMASTER UGM: scriptable, JSON-first,
agent-friendly. Built for students who prefer the shell, scripts, and AI agents
over a web UI.

**Status: pre-alpha.** There is no released binary yet. Offline diagnostics and
shell completions are available; authentication and SIMASTER data commands are
not implemented.

[![CI](https://github.com/Andhiki/simaster/actions/workflows/ci.yml/badge.svg)](https://github.com/Andhiki/simaster/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](Cargo.toml)

## Why

- **Terminal-first.** Read your profile, schedule, grades, and attendance
  without leaving the shell.
- **Scriptable.** Human-readable output on a TTY, stable JSON when you ask for
  it, and documented exit codes.
- **Agent-friendly.** A predictable command grammar, a versioned JSON envelope,
  and no interactive prompts when JSON output is requested.
- **Small surface.** Read-only by default. Write operations are not available.

## Install

No release has been published. Build from source:

```bash
cargo install --git https://github.com/Andhiki/simaster
```

Rust 1.85 or newer is required. The crate uses edition 2024.

## Quickstart

```bash
# Inspect the local environment without network access.
simaster doctor
simaster doctor --json | jq .

# Generate completions from the live command tree.
simaster completions bash > ~/.simaster.bash

simaster --help
simaster --version
```

`doctor` is entirely offline. It never contacts SIMASTER. `auth` and `profile`
are registered commands that return `not implemented` with exit code 1;
`whoami` is not registered yet. Browser authentication cannot complete until a
UGM ID client is registered; the web session with CAS is the fallback design.
Neither path is implemented.

## Commands

| Command | Status | Notes |
|---|---|---|
| `simaster doctor` | available | Offline environment and config checks. |
| `simaster doctor --json` | available | Stable envelope, `schema_version: 1`. |
| `simaster completions <shell>` | available | `bash`, `zsh`, `fish`, `powershell`, `elvish`. |
| `simaster auth login` | not implemented | Browser PKCE primary; web session + CAS fallback. |
| `simaster auth status` | not implemented | Provider, username, scopes, expiry, storage; never token values. |
| `simaster auth logout` | not implemented | Local delete plus browser logout when possible; reports partial revocation. |
| `simaster profile show` | not implemented | Full profile through UGM ID. |
| `simaster whoami` | not registered | Identity through UGM ID. |

The `auth login` parser accepts `--method auto|browser|web|sso`, `--username`,
and `--password-stdin`; it fails before doing any work. See
[`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md) for the full contract.

## Authentication

The primary strategy is the **official UGM ID OAuth 2.0/OIDC Authorization Code
flow with PKCE S256**, a public/native client, and a loopback redirect. It is
used when `SIMASTER_CLIENT_ID` is configured. The flow goes through UGM CAS SSO
and a consent screen, uses `state` and `nonce`, binds a one-shot listener to
`127.0.0.1:53682`, validates the ID token with the published JWKS, and never
handles the UGM password.

The **web session with CAS fallback** is used when no UGM ID client is
configured; it is the only path available until DTI registers one.

The **vNext gateway is a data adapter only**. It is not an authentication
provider. It may accept the official UGM ID access token; that is `unverified`.
If it does not, the CLI uses the web data adapter.

The official mobile app's embedded client secret is never copied, bundled,
shipped, logged, or documented as a value, and its custom-scheme redirect is not
used. No client secret is shipped for a public client.

See [`docs/AUTH.md`](docs/AUTH.md) and
[`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md).

Credentials are never accepted as command-line arguments. Passwords for the web
fallback are read from a prompt or stdin. Tokens, cookies, authorization codes,
and PKCE verifiers are never logged.

## Agentic usage

- Add `--json` to get machine-readable output. The envelope is versioned with
  `schema_version`; see [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md).
- Diagnostics go to stderr; structured output goes to stdout.
- Exit codes are 0 success, 1 generic or not implemented, and 2 usage today;
  3-6 are reserved for auth, network, upstream, and forbidden errors. See
  [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md).
- The CLI is read-only. Write commands and a read-only `api` escape hatch are
  tracked in [GitHub Issues](https://github.com/pendopo-dev/simaster/issues).
- Browser login is interactive; `--json` does not remove the browser step.
- Set `NO_COLOR=1` for stable terminal output. `RUST_LOG` controls stderr
  logging.

## Documentation

| Document | Purpose |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Layers, module map, current command flow, error model, testing. |
| [`docs/AUTH.md`](docs/AUTH.md) | Authentication methods, flows, endpoints, storage, threat model. |
| [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md) | Command grammar, flags, JSON envelope, exit codes, env vars. |
| [`docs/adr/0001-single-crate.md`](docs/adr/0001-single-crate.md) | Why the project uses one crate. |
| [`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md) | Why UGM ID PKCE is primary and web remains the fallback. |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Setup, tests, branches, commits, review, release. |
| [`GOVERNANCE.md`](GOVERNANCE.md) | Roles, consensus, RFCs, maintainer lifecycle. |
| [`SECURITY.md`](SECURITY.md) | Reporting, scope, credential rules, disclosure. |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Contributor Covenant 2.1. |

Task state lives in [GitHub Issues](https://github.com/pendopo-dev/simaster/issues).

## Contributing

Contributions are welcome. Read [`CONTRIBUTING.md`](CONTRIBUTING.md) first.
Small documentation fixes can go straight to a pull request; behavior changes
need an issue and a test. Authentication, endpoint, and security changes need a
second reviewer.

## Security

Do not open a public issue for a vulnerability. Use GitHub Security Advisories
as described in [`SECURITY.md`](SECURITY.md). Never include credentials,
cookies, tokens, authorization codes, or real student data in an issue, pull
request, fixture, or log.

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE))
- MIT License ([`LICENSE-MIT`](LICENSE-MIT))

at your option. Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this project is dual-licensed under
the same terms, without additional conditions.

## Disclaimer

`simaster` is an unofficial project. It is **not affiliated with, endorsed by,
or supported by Universitas Gadjah Mada (UGM)**. Use it only with your own
account, in compliance with UGM policies and applicable law. The project does
not provide credentials, does not bypass authentication or authorization, and
comes with no warranty. You are responsible for the requests you make and for
protecting your own data.

## Acknowledgements

The official UGM ID OAuth 2.0/OIDC documentation is the primary source for the
authentication design:

- [UGM ID Swagger UI](https://oauth.simaster.ugm.ac.id/docs)
- [UGM ID OpenAPI 3.1 specification](https://oauth.simaster.ugm.ac.id/openapi.json)
- [UGM ID OIDC discovery](https://oauth.simaster.ugm.ac.id/.well-known/openid-configuration)
- [UGM ID public health check](https://oauth.simaster.ugm.ac.id/health)

Prior-art static analysis of the official mobile app, especially
[`gafnaa/vnext-reveng`](https://github.com/gafnaa/vnext-reveng), is secondary
evidence for the vNext **data adapter**, not for authentication. Community KKN
automation work informed the web-session and CAS fallback design. Official UGM
ID behavior is `documented by UGM` and `live-tested: no` except the public
health check; prior-art endpoint claims are `unverified`. Static analysis is
evidence of client code, not server behavior. The mobile app's client ID,
embedded secret, tokens, cookies, and custom-scheme redirect are not included.
