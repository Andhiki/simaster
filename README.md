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

`simaster auth ...` and `simaster profile show` are registered commands but
return a clear `not implemented` error. Their real behavior is planned for M2
and M3. See [`docs/ROADMAP.md`](docs/ROADMAP.md).

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

# Planned for M2: sign in with the web session flow, with CAS fallback.
simaster auth login

# Planned for M3: read the authenticated student's profile.
simaster profile show
```

`doctor` is entirely offline. It never contacts SIMASTER. The auth and profile
commands are marked **planned** and currently fail with exit code 1 and
`not implemented`.

## Commands

| Command | Status | Notes |
|---|---|---|
| `simaster doctor` | available | Offline environment and config checks. |
| `simaster doctor --json` | available | Stable envelope, `schema_version: 1`. |
| `simaster completions <shell>` | available | `bash`, `zsh`, `fish`, `powershell`, `elvish`. |
| `simaster auth login` | planned (M2) | Web session first, CAS fallback. Returns not implemented now. |
| `simaster auth status` | planned (M2) | Reports the stored session. Returns not implemented now. |
| `simaster auth logout` | planned (M2) | Removes stored credentials and sessions. Returns not implemented now. |
| `simaster whoami` | planned (M2) | Shows the authenticated identity. |
| `simaster profile show` | planned (M3) | Profile read; returns not implemented now. |
| `simaster schedule today` | planned (M3) | Today's schedule. |
| `simaster schedule week` | planned (M3) | One week of schedule. |
| `simaster grades list` | planned (M3) | Study results by semester. |
| `simaster attendance summary` | planned (M3) | Attendance recap. |
| `simaster api` | planned (M4) | Read-only service access for scripts. |
| MCP server | planned (M4) | Separate process, not part of the single-binary CLI yet. |
| Write commands | not before M5 | Opt-in, dry-run first, explicit confirmation. |

Authentication methods:

| Method | Status | Default |
|---|---|---|
| Web session with CAS fallback | default, planned (M2) | yes |
| vNext OAuth | experimental, opt-in, planned (M2) | no |
| Browser SSO with PKCE | target for v1.0 | no |

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

## Authentication

The default authentication strategy is a **web session with a CAS fallback**.
The CLI signs in to the SIMASTER web session endpoint and falls back to the UGM
central authentication service when the direct flow is unavailable.

The official mobile app's vNext OAuth flow is documented as an **experimental,
opt-in** provider. The app's embedded client secret is never copied, bundled,
shipped, or documented as a value. The vNext provider requires a user-supplied
authorized client, supplied through the environment or the operating-system
keyring.

A browser SSO flow with PKCE and a loopback redirect is the target for v1.0.
It is currently blocked on a client registration that the project does not
control. See [`docs/AUTH.md`](docs/AUTH.md) and
[`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md).

Credentials are never accepted as command-line arguments. Passwords are read
from a prompt or stdin. Tokens and cookies are never logged.

## Agentic usage

- Add `--json` to get machine-readable output. The envelope is versioned with
  `schema_version`; see [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md).
- Diagnostics go to stderr; structured output goes to stdout.
- Errors use documented exit codes: 0 success, 1 generic or not implemented,
  2 usage, 3 auth, 4 network or rate limit, 5 upstream changed, 6 forbidden.
- The CLI is read-only by default. Future write commands require an explicit
  opt-in flag, a dry run, and confirmation.
- A read-only `api` escape hatch and a separate MCP server are planned for M4.
  They will reuse the same library and contract rather than bypassing it.
- Set `NO_COLOR=1` for stable terminal output. `RUST_LOG` controls stderr
  logging.

## Documentation

| Document | Purpose |
|---|---|
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Layers, module map, data flow, testing, MCP boundary. |
| [`docs/AUTH.md`](docs/AUTH.md) | Authentication options, flows, endpoints, threat model. |
| [`docs/API-MAP.md`](docs/API-MAP.md) | Endpoint registry, gateway envelope, service labels, stability. |
| [`docs/CLI-CONTRACT.md`](docs/CLI-CONTRACT.md) | Command grammar, flags, JSON envelope, exit codes, env vars. |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Milestones M0–M6, exit criteria, non-goals. |
| [`docs/REPO-SETUP.md`](docs/REPO-SETUP.md) | Repository settings runbook for the owner. |
| [`docs/adr/0001-single-crate.md`](docs/adr/0001-single-crate.md) | Why the project starts as one crate. |
| [`docs/adr/0002-auth-strategy.md`](docs/adr/0002-auth-strategy.md) | Why web session is the default and OAuth is opt-in. |
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
cookies, tokens, or real student data in an issue, pull request, fixture, or
log.

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

The endpoint inventory and authentication analysis were informed by static
analysis of the official mobile app and by public prior art, especially
[`gafnaa/vnext-reveng`](https://github.com/gafnaa/vnext-reveng). Community KKN
automation work informed the web-session and CAS fallback design. All endpoint
claims in this repository are marked `unverified` unless a live test is cited;
static analysis is evidence of client code, not of server behavior.
