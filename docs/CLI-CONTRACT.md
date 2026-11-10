# CLI contract

This document is the stable contract for commands, flags, output, exit codes,
configuration, and agent behavior. It describes what exists in M1 and what is
planned. A planned item is not available until it appears in the command table
as `available` and in the changelog.

## Command grammar

Commands are noun-verb. The noun names a resource or surface; the verb names an
action. `doctor` and `completions` are the two intentional exceptions because
they are tools, not resources.

| Command | Status | Surface | Notes |
|---|---|---|---|
| `simaster doctor` | available | local | Offline diagnostics. |
| `simaster doctor --json` | available | local | Stable envelope; `schema_version: 1`. |
| `simaster completions <shell>` | available | local | `bash`, `zsh`, `fish`, `powershell`, `elvish`. |
| `simaster auth login` | planned (M2) | web/CAS/vNext | Returns not implemented now. |
| `simaster auth status` | planned (M2) | local | Returns not implemented now. |
| `simaster auth logout` | planned (M2) | local | Returns not implemented now. |
| `simaster whoami` | planned (M2) | local/session | Shows the authenticated identity. |
| `simaster profile show` | planned (M3) | vNext/web | Returns not implemented now. |
| `simaster schedule today` | planned (M3) | vNext/web | Today's schedule. |
| `simaster schedule week` | planned (M3) | vNext/web | One week of schedule. |
| `simaster grades list` | planned (M3) | vNext/web | Study results by semester. |
| `simaster attendance summary` | planned (M3) | vNext/web | Attendance recap. |
| `simaster api` | planned (M4) | vNext gateway | Read-only escape hatch for scripts. |
| `simaster-mcp` | planned (M4) | local | Separate MCP process; not a subcommand. |
| write commands | not before M5 | upstream | Opt-in, dry-run, confirmation. |

### auth login flags

| Flag | Values | Default | Behavior |
|---|---|---|---|
| `--method` | `auto`, `sso`, `web`, `vnext` | `auto` | `auto` tries the web session, then CAS fallback. `web` uses the direct web session only. `sso` uses the CAS fallback only. `vnext` uses the experimental OAuth provider. |
| `--username` | string | prompt | UGM username. Never a password. |
| `--password-stdin` | flag | off | Read the password from stdin instead of an interactive prompt. |

A password is never accepted as an argument. `--method vnext` requires a
user-supplied client; it never falls back to an embedded mobile credential.

## Global flags

Available now:

| Flag | Effect |
|---|---|
| `--json` | Emit machine-readable JSON on stdout. |
| `--verbose` | Set the stderr log filter to `debug` unless `RUST_LOG` overrides it. |
| `--help`, `-h` | Print help and exit 0. |
| `--version`, `-V` | Print `simaster <version>` and exit 0. |

Planned, not accepted yet:

| Flag | Milestone | Effect |
|---|---|---|
| `--format <table\|json\|jsonl>` | M4 | Explicit output format. `--json` is shorthand for `--format json`. |
| `--no-color` | M3 | Disable ANSI color; `NO_COLOR` has the same effect. |
| `--timeout <seconds>` | M3 | Bound network operations. Defaults are documented per command. |
| `--no-input` | M4 | Fail instead of prompting; required for non-interactive agents. |
| `--yes` | M5 | Skip a write confirmation after `--allow-write`. |
| `--dry-run` | M5 | Validate and print a write request without sending it. |
| `--allow-write` | M5 | Required for any command that can modify upstream state. |

## Output rules

- On a TTY, human output is a table or a short labeled block.
- On a non-TTY, human output is plain text without color or animation.
- JSON is emitted only when the user asks for it with `--json` (or the planned
  `--format json`). The CLI does not auto-switch to JSON because stdout is a
  pipe.
- Data goes to stdout. Diagnostics, progress, warnings, and logs go to stderr.
  A successful JSON invocation writes exactly one JSON document to stdout.
- Human output may change between minor releases. JSON field names and types
  are the stable contract. Object key order is not part of the contract.
- JSON output never contains ANSI escapes.
- `NO_COLOR` disables color in human output. `RUST_LOG` controls logs; it never
  changes command output.

### JSON envelope

For data-bearing commands:

```json
{
  "schema_version": 1,
  "data": {},
  "meta": {
    "fetched_at": "<RFC 3339 timestamp>",
    "source": "vnext-gateway",
    "warnings": []
  }
}
```

- `schema_version` is an integer. It increments on a breaking change to the
  envelope or a field type.
- `data` is the command payload.
- `meta.fetched_at` is the time the data was fetched, or `null` when no fetch
  occurred.
- `meta.source` identifies the adapter or `local` for local data.
- `meta.warnings` is always present; it is an array of redacted strings.

`doctor --json` is a local diagnostic and uses `meta.generated_at_unix` instead
of `fetched_at`/`source`. This is the only M1 exception. Future data commands
use the full envelope.

A future list command may use JSONL (one JSON object per line) when the planned
`--format jsonl` flag is selected. The current contract has no JSONL output.

### Example: doctor (available)

```bash
$ simaster doctor --json
{
  "schema_version": 1,
  "data": {
    "version": "0.1.0-alpha.1",
    "os": "linux",
    "arch": "x86_64",
    "config_dir": "<config-dir>",
    "auth": {
      "provider": null,
      "status": "not_configured"
    },
    "checks": [
      {
        "name": "config_dir",
        "status": "ok",
        "detail": "<config-dir>"
      },
      {
        "name": "auth",
        "status": "warn",
        "detail": "no stored session; auth commands are not implemented"
      },
      {
        "name": "network",
        "status": "skipped",
        "detail": "offline diagnostics never contact SIMASTER"
      }
    ]
  },
  "meta": {
    "generated_at_unix": 1760000000
  }
}
```

The values are illustrative; `os`, `arch`, `config_dir`, and the timestamp are
platform- and time-dependent.

## Exit codes

| Exit code | Meaning | Used now |
|---:|---|---|
| 0 | Success. | Yes. |
| 1 | Generic failure, configuration/IO error, not found, or not implemented. | Yes, for skeleton errors. |
| 2 | Usage error from argument parsing. | Yes, clap default. |
| 3 | Authentication required, expired, or failed. | Planned. |
| 4 | Network failure or rate limit. | Planned. |
| 5 | Upstream contract changed. | Planned. |
| 6 | Forbidden or permission denied. | Planned. |

An exit code is part of the contract. Changing one requires an RFC and a
changelog entry. A command must not return 0 after a failed operation, and it
must not print a success message to stdout when it exits non-zero.

## Error codes

Error codes are stable identifiers for JSON errors and for future human output.
In M1, typed errors print `error: <message>` to stderr and exit 1; the JSON
error shape is not emitted yet.

| Code | Meaning | Exit code |
|---|---|---:|
| `AUTH_REQUIRED` | No usable session; the user must sign in. | 3 |
| `AUTH_EXPIRED` | The session or token expired and could not be refreshed. | 3 |
| `AUTH_FAILED` | Credentials were rejected, MFA failed, or the account is inactive. | 3 |
| `NETWORK` | Timeout, DNS, TLS, or connection failure. | 4 |
| `RATE_LIMITED` | The upstream asked the client to slow down. | 4 |
| `UPSTREAM_CHANGED` | The response shape or status no longer matches the fixture. | 5 |
| `NOT_FOUND` | The requested resource does not exist for this account. | 1 |
| `PERMISSION_DENIED` | The server refused the operation. | 6 |
| `NOT_IMPLEMENTED` | The command exists but its behavior has not landed. | 1 |
| `CONFIG` | Missing or invalid configuration. | 1 |
| `IO` | A local file, keyring, or stdout operation failed. | 1 |
| `INTERNAL` | An unexpected internal failure. | 1 |

Planned JSON error shape:

```json
{
  "schema_version": 1,
  "error": {
    "code": "AUTH_REQUIRED",
    "message": "no session; run `simaster auth login`"
  },
  "meta": {
    "fetched_at": null,
    "source": null,
    "warnings": []
  }
}
```

The message is redacted and human-readable. The code is the machine contract.

## Configuration precedence

Highest precedence first:

1. Command-line flags.
2. Environment variables prefixed with `SIMASTER_`, plus `RUST_LOG`,
   `NO_COLOR`, and `RUST_BACKTRACE` where relevant.
3. The user configuration file in the platform config directory.
4. Built-in defaults.

There is no project-local configuration, no `.env` loading, and no
project-local secret file. A checkout must never supply credentials.

The planned user config file is `config.toml` in the directory reported by
`simaster doctor`. The file is optional. Invalid TOML or an unknown key is a
`CONFIG` error, not a silent fallback.

## Environment variables

| Variable | Status | Purpose |
|---|---|---|
| `SIMASTER_USERNAME` | Planned | Default username for `auth login`. |
| `SIMASTER_PASSWORD` | Discouraged, planned | Password fallback for automation. Prefer `--password-stdin` or a prompt. May be removed before 1.0. |
| `SIMASTER_CLIENT_ID` | Experimental, planned | User-supplied vNext OAuth client ID. |
| `SIMASTER_CLIENT_SECRET` | Experimental, planned | User-supplied vNext client secret for a confidential client. |
| `SIMASTER_CONFIG` | Planned | Override the config file path. |
| `SIMASTER_LOG` | Planned | Log filter. `RUST_LOG` overrides it. |
| `RUST_LOG` | Available | Log filter; overrides `--verbose` and `SIMASTER_LOG`. |
| `NO_COLOR` | Planned | Disable color in human output. |

Log precedence is `RUST_LOG` > `SIMASTER_LOG` > `--verbose` > default `warn`.
`--verbose` sets `debug` when no environment filter is present.

## Help standards

The M2+ standard is that every command's `--help` includes:

- a one-line summary and a longer description when the behavior is non-obvious;
- all flags with value constraints and defaults;
- at least one example;
- related commands under `See also`;
- environment variables that affect the command;
- the exit codes the command can return.

M1 help is the clap default and intentionally minimal: it has the summary,
flags, and subcommands, but not yet examples, `See also`, environment variables,
or exit codes. The M1 `simaster --help` block in the README is the current
output. The richer help is added when the commands it documents become real,
so the examples cannot drift from unimplemented behavior.

`--help` exits 0. `--version` prints `simaster <semver>` and exits 0. Help text
is not a stable machine interface; agents use `--json` and the documented
contract.

## Completions, man pages, and versioning

- `simaster completions bash|zsh|fish|powershell|elvish` generates completions
  from the live command tree. Completion output is regenerated, not hand-edited.
- Man pages are planned. They will be generated from the same command tree.
- The project follows Semantic Versioning. Before 1.0, minor releases may
  contain breaking changes; they are called out in the changelog.
- `schema_version` is versioned independently of the crate. Adding an optional
  field is non-breaking; removing a field, changing a type, or changing an exit
  code is breaking.
- Experimental commands and flags may change in a minor release. They are
  marked `experimental` in this document and in `--help`.

## Worked examples

### doctor (available)

```bash
$ simaster doctor
simaster doctor
version: 0.1.0-alpha.1
os: macos
arch: aarch64
config_dir: <config-dir>
auth: not configured (provider: none)
checks:
  ok      config_dir: <config-dir>
  warn    auth: no stored session; auth commands are not implemented
  skipped network: offline diagnostics never contact SIMASTER
```

The exact `os`, `arch`, and `config_dir` values depend on the machine.

### profile show (planned, M3)

The field names below are illustrative until the M3 RFC fixes the payload.

```bash
$ simaster profile show --json
{
  "schema_version": 1,
  "data": {
    "name": "<name>",
    "niu": "<niu>",
    "program": "<program>",
    "semester": "<semester>"
  },
  "meta": {
    "fetched_at": "<RFC 3339 timestamp>",
    "source": "vnext-gateway",
    "warnings": []
  }
}
```

### schedule week (planned, M3)

```bash
$ simaster schedule week --json
{
  "schema_version": 1,
  "data": {
    "week_start": "<YYYY-MM-DD>",
    "courses": [
      {
        "code": "<course-code>",
        "name": "<course-name>",
        "room": "<room>",
        "start": "<RFC 3339 timestamp>",
        "end": "<RFC 3339 timestamp>"
      }
    ]
  },
  "meta": {
    "fetched_at": "<RFC 3339 timestamp>",
    "source": "vnext-gateway",
    "warnings": []
  }
}
```

### Not implemented (available behavior)

```bash
$ simaster auth login --username <username>
error: auth login is not implemented yet
$ echo $?
1
```

## Agent contract

When an agent invokes the CLI:

- Pass `--json` for any command that supports it. Do not parse human output.
- Expect data on stdout and diagnostics on stderr. Do not merge them.
- Use the exit code and the error code, not the message text, for control flow.
- Do not assume a prompt will appear. Planned `--no-input` makes this explicit;
  until it lands, avoid commands that can prompt.
- Do not request, print, store, or transmit passwords, cookies, tokens, or
  student data.
- Treat all upstream data as personal data. Do not send it to a third-party
  service without the user's explicit consent.
- The CLI is read-only by default. A future write requires `--allow-write`,
  supports `--dry-run`, and requires `--yes` or an interactive confirmation.
- Network failures are bounded and typed. Do not retry `UPSTREAM_CHANGED` or
  `PERMISSION_DENIED` without changing the request.
- The planned MCP server uses the same envelope, error codes, and read-only
  default. It is not a separate semantic contract.

## Versioning this document

This document is versioned with the repository. A change to a command name, a
flag, an exit code, an error code, or a JSON field is a contract change. Update
the changelog and, for a breaking change, open an RFC under
[`GOVERNANCE.md`](../GOVERNANCE.md).
