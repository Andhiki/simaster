# simaster

Unofficial terminal-first CLI for SIMASTER UGM: scriptable, JSON-first,
agent-friendly. Built for students who prefer the shell, scripts, and AI agents
over a web UI.

**Status: pre-alpha, docs-first.** There is no released binary yet.
Authentication and SIMASTER data commands are planned.

[![CI](https://github.com/Andhiki/simaster/actions/workflows/ci.yml/badge.svg)](https://github.com/Andhiki/simaster/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](Cargo.toml)

## What it is

`simaster` is a single-binary Rust CLI for the SIMASTER UGM student portal. It
is designed to make routine academic data readable from a terminal, a shell
script, or an agent without driving a browser by hand.

## Why

- **Terminal-first.** Read your profile, schedule, grades, and attendance
  without leaving the shell.
- **Scriptable.** Human-readable tables on a TTY, stable JSON when you ask for
  it, and documented exit codes.
- **Agent-friendly.** A predictable command grammar, a versioned JSON envelope,
  and no interactive prompts when JSON output is requested.
- **Small surface.** Read-only by default. Write operations are a later,
  explicitly opt-in milestone.

## Status

The repository is at **M1: walking skeleton + CI**. The following commands are
available now:

- `simaster doctor` — offline diagnostics and config directory discovery.
- `simaster doctor --json` — the same diagnostics with `schema_version: 1`.
- `simaster completions <shell>` — completions for bash, zsh, fish, PowerShell,
  and Elvish.
- `simaster --help`, `simaster --version`.

`simaster auth ...`, `simaster whoami`, and `simaster profile show` are the M2
profile proof of concept. `auth` and `profile` are registered commands but return
a clear `not implemented` error today; `whoami` is not registered yet. See the
[M2 milestone](https://github.com/Andhiki/simaster/milestone/1).

## Install

No release has been published. Build from source:

```bash
cargo install --git https://github.com/Andhiki/simaster
```

Or clone and build a release binary:

```bash
git clone https://github.com/Andhiki/simaster
cd simaster
cargo build --release
./target/release/simaster --version
```

Rust 1.85 or newer is required. The crate uses edition 2024.

## Quickstart

```bash
# Available now: inspect the local environment without network access.
simaster doctor
simaster doctor --json | jq .

# Planned for M2: browser OAuth2 + PKCE when a DTI-registered client ID is set.
export SIMASTER_CLIENT_ID="<client-id>"
simaster auth login
simaster whoami
simaster profile show
```

`doctor` is entirely offline. It never contacts SIMASTER. The auth, whoami, and
profile commands are marked **planned** and currently fail with exit code 1 and
`not implemented`; `whoami` is not in the M1 command tree yet. Without
`SIMASTER_CLIENT_ID`, `auth login --method auto` uses the web session with CAS
fallback instead of the browser PKCE flow.

## Commands

| Command | Status | Notes |
|---|---|---|
| `simaster doctor` | available | Offline environment and config checks. |
| `simaster doctor --json` | available | Stable envelope, `schema_version: 1`. |
| `simaster completions <shell>` | available | `bash`, `zsh`, `fish`, `powershell`, `elvish`. |
| `simaster auth login` | planned (M2) | Browser PKCE primary; web session + CAS fallback. Returns not implemented now. |
| `simaster auth status` | planned (M2) | Provider, username, scopes, expiry, storage; never token values. |
| `simaster auth logout` | planned (M2) | Local delete + browser logout when possible; reports partial revocation. |
| `simaster whoami` | planned (M2) | Identity via UGM ID `/api/me`. |
| `simaster profile show` | planned (M2) | Full profile via UGM ID `/user/me`. |
| `simaster schedule today` | planned (M3) | Today's schedule, using a data adapter. |
| `simaster schedule week` | planned (M3) | One week of schedule. |
| `simaster grades list` | planned (M3) | Study results by semester. |
| `simaster attendance summary` | planned (M3) | Attendance recap. |
| `simaster api` | planned (M4) | Read-only service access for scripts. |
| MCP server | planned (M4) | Separate process, not part of the single-binary CLI yet. |
| Write commands | not before M5 | Opt-in, dry-run first, explicit confirmation. |

Authentication methods:

| Method | Status | Default |
|---|---|---|
| Browser OAuth2 + PKCE S256 (UGM ID) | primary when `SIMASTER_CLIENT_ID` is configured; planned (M2) | yes, when configured |
| Web session + CAS fallback | fallback and default when no client ID; planned (M2) | yes, otherwise |
| vNext gateway | data adapter only; official token acceptance `unverified`; planned (M3) | not an auth method |
| Mobile client secret | rejected; never used | no |

## Current help

```text
Unofficial terminal-first CLI for SIMASTER UGM

Usage: simaster [OPTIONS] <COMMAND>

Commands:
  auth         Authenticate with SIMASTER
  profile      Inspect your SIMASTER profile
  doctor       Run offline diagnostics
  completions  Generate shell completions
  help         Print this message or the help of the given subcommand(s)

Options:
      --json     Emit machine-readable JSON
      --verbose  Enable debug logging to stderr
  -h, --help     Print help
  -V, --version  Print version
```

`simaster auth login --help` currently exposes the revised method contract:

```text
Sign in to SIMASTER

Usage: simaster auth login [OPTIONS]

Options:
      --json
          Emit machine-readable JSON

      --method <METHOD>
          Authentication method.

          `auto` uses browser PKCE when `SIMASTER_CLIENT_ID` is configured; otherwise it falls back
          to the web session and CAS. `browser` requires `SIMASTER_CLIENT_ID`.

          [default: auto]
          [possible values: auto, browser, web, sso]

      --username <USERNAME>
          UGM username for the web session fallback

      --verbose
          Enable debug logging to stderr

      --password-stdin
          Read the web-fallback password from stdin instead of prompting

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Authentication

The primary authentication strategy is the **official UGM ID OAuth 2.0/OIDC
Authorization Code flow with PKCE S256**, a public/native client, and a loopback
redirect. It is used when `SIMASTER_CLIENT_ID` is configured. The browser
redirect goes through UGM CAS SSO and a consent screen. The CLI sends `state`
and `nonce`, binds a one-shot listener to `127.0.0.1:53682`, validates the ID
token with the published JWKS, and never sees or stores the UGM password.

The **web session with CAS fallback** remains the fallback and the default when
no UGM ID client is configured. It is the only path available before DTI
registers a client.

The **vNext gateway is a data adapter only**. It is not an authentication
provider. It may accept the official UGM ID access token; that is `unverified`.
If it does not, the CLI uses the web data adapter.

The official mobile app's embedded client secret is never copied, bundled,
shipped, logged, or documented as a value. The app's custom-scheme redirect is
not used by this CLI. No client secret is shipped for a public client.

See [`docs/UGM-ID-API.md`](docs/UGM-ID-API.md), [`docs/AUTH.md`](docs/AUTH.md),
and [`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md).

Credentials are never accepted as command-line arguments. Passwords for the web
fallback are read from a prompt or stdin. Tokens, cookies, authorization codes,
and PKCE verifiers are never logged.

## Agentic usage

- Add `--json` to get machine-readable output. The envelope is versioned with
  `schema_version`; see [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md).
- Diagnostics go to stderr; structured output goes to stdout.
- Errors use documented exit codes: 0 success, 1 generic or not implemented,
  2 usage, 3 auth, 4 network or rate limit, 5 upstream changed, 6 forbidden.
- The CLI is read-only by default. Future write commands require an explicit
  opt-in flag, a dry run, and confirmation.
- Browser login is interactive by nature. `--json` does not remove the browser
  step; `--no-browser` will print the authorization URL for a user to open.
- A read-only `api` escape hatch and a separate MCP server are planned for M4.
  They will reuse the same library and contract rather than bypassing it.
- Set `NO_COLOR=1` for stable terminal output. `RUST_LOG` controls stderr
  logging.

## Documentation

| Document | Purpose |
|---|---|
| [`docs/UGM-ID-API.md`](docs/UGM-ID-API.md) | Official UGM ID OAuth2/OIDC reference, endpoints, PKCE, registration, DTI email. |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Layers, module map, data flow, testing, MCP boundary. |
| [`docs/AUTH.md`](docs/AUTH.md) | Authentication options, flows, endpoints, threat model. |
| [`docs/API-MAP.md`](docs/API-MAP.md) | Level map, endpoint registry, gateway envelope, service labels, stability. |
| [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md) | Command grammar, flags, JSON envelope, exit codes, env vars. |
| [Milestones](https://github.com/Andhiki/simaster/milestones) | M2–M6 phases, their issues, and exit criteria. |
| [`docs/REPO-SETUP.md`](docs/REPO-SETUP.md) | Repository settings runbook and UGM ID registration checklist. |
| [`docs/adr/0001-single-crate.md`](docs/adr/0001-single-crate.md) | Why the project starts as one crate. |
| [`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md) | Why UGM ID PKCE is primary and web remains the fallback. |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | Setup, tests, branches, commits, review, release. |
| [`GOVERNANCE.md`](GOVERNANCE.md) | Roles, consensus, RFCs, maintainer lifecycle. |
| [`SECURITY.md`](SECURITY.md) | Reporting, scope, credential rules, disclosure. |
| [`SUPPORT.md`](SUPPORT.md) | Where to ask questions and report bugs. |
| [`CHANGELOG.md`](CHANGELOG.md) | Notable changes, Keep a Changelog format. |
| [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) | Contributor Covenant 2.1. |

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
- [UGM ID development server](https://oauth.dev.ugm.ac.id)

Prior-art static analysis of the official mobile app, especially
[`gafnaa/vnext-reveng`](https://github.com/gafnaa/vnext-reveng), is secondary
evidence for the vNext **data adapter**, not for authentication. Community KKN
automation work informed the web-session and CAS fallback design.

Official UGM ID behavior is marked `documented by UGM` and `live-tested: no`
(except the public health check). Prior-art endpoint claims are marked
`unverified`. Static analysis is evidence of client code, not of server
behavior. The mobile app's client ID, embedded secret, tokens, cookies, and the
custom-scheme redirect as a usable credential are not included.
