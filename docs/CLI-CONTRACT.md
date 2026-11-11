# CLI contract

The stable contract for the commands, flags, output, exit codes, configuration, and agent behavior that exist today. Contracts for work that is not implemented are tracked in [GitHub Issues](https://github.com/pendopo-dev/simaster/issues); a command is available only when it appears here as `available` and in tests.

## Command grammar

Commands are noun-verb; the noun names a resource or surface and the verb names an action. `doctor` and `completions` are intentional exceptions because they are tools, not resources.

| Command | Status | Surface | Notes |
|---|---|---|---|
| `simaster doctor` | available | local | Offline diagnostics. |
| `simaster doctor --json` | available | local | Stable envelope; `schema_version: 1`. |
| `simaster completions <shell>` | available | local | `bash`, `zsh`, `fish`, `powershell`, `elvish`. |
| `simaster auth login` | not implemented | UGM ID, web/CAS | Returns `not implemented`, exit 1. |
| `simaster auth status` | not implemented | local | Returns `not implemented`. |
| `simaster auth logout` | not implemented | local/browser | Returns `not implemented`. |
| `simaster profile show` | not implemented | UGM ID | Returns `not implemented`. |
| `simaster whoami` | not registered | UGM ID | Not in the command tree. |

The `auth login` parser accepts:

| Flag | Values | Default | Behavior |
|---|---|---|---|
| `--method` | `auto`, `browser`, `web`, `sso` | `auto` | `auto` selects browser PKCE when `SIMASTER_CLIENT_ID` is configured; otherwise web session with CAS. `browser` requires a client ID. |
| `--username` | string | prompt | UGM username for the web fallback; never a password. |
| `--password-stdin` | flag | off | Read the web-fallback password from stdin instead of prompting. |

A password is never accepted as an argument. vNext is a data adapter, not an authentication method. `auto` does not downgrade in the middle of a login.

## Global flags

| Flag | Effect |
|---|---|
| `--json` | Emit machine-readable JSON on stdout. |
| `--verbose` | Set the stderr log filter to `debug` unless `RUST_LOG` overrides it. |
| `--help`, `-h` | Print help and exit 0. |
| `--version`, `-V` | Print `simaster <version>` and exit 0. |

A flag not in this table is not accepted. New flags update this table and the command help in the same change.

## Output rules

- Human output is a table or short labeled block on a TTY, plain text without color or animation otherwise.
- JSON is emitted only when `--json` is passed; the CLI never auto-switches because stdout is a pipe.
- Data goes to stdout; diagnostics, progress, warnings, and logs go to stderr. A successful JSON invocation writes exactly one document.
- Human output may change between minor releases; JSON field names and types are the contract, key order is not. JSON never contains ANSI escapes.
- `NO_COLOR` disables color in human output; `RUST_LOG` never changes command output.

## JSON envelope

```json
{
  "schema_version": 1,
  "data": {},
  "meta": {
    "fetched_at": "<RFC 3339 timestamp>",
    "source": "ugm-id",
    "warnings": []
  }
}
```

`schema_version` increments on a breaking envelope or field change. `data` is the payload. `meta.fetched_at` is `null` when no fetch occurred. `meta.source` is `ugm-id`, `vnext-gateway`, `web`, or `local`. `meta.warnings` is always present and contains redacted strings. `doctor --json` is the exception: a local diagnostic with `data` and `meta.generated_at_unix`.

## Exit codes

| Exit code | Meaning | Used now |
|---:|---|---|
| 0 | Success. | Yes. |
| 1 | Generic failure, configuration/IO error, not found, or not implemented. | Yes. |
| 2 | Usage error from argument parsing. | Yes, clap default. |
| 3 | Authentication required, expired, or failed. | Reserved. |
| 4 | Network failure or rate limit. | Reserved. |
| 5 | Upstream contract changed. | Reserved. |
| 6 | Forbidden or permission denied. | Reserved. |

Changing an exit code requires an RFC; a command must not return 0 after a failed operation.

## Error codes

| Code | Meaning | Exit code |
|---|---|---:|
| `AUTH_REQUIRED` | No usable session; the user must sign in. | 3 |
| `AUTH_EXPIRED` | The session or token expired and could not be refreshed. | 3 |
| `AUTH_FAILED` | Credentials rejected, MFA failed, or account inactive. | 3 |
| `NETWORK` | Timeout, DNS, TLS, or connection failure. | 4 |
| `RATE_LIMITED` | The upstream asked the client to slow down. | 4 |
| `UPSTREAM_CHANGED` | The response shape or status no longer matches the fixture. | 5 |
| `NOT_FOUND` | The requested resource does not exist for this account. | 1 |
| `PERMISSION_DENIED` | The server refused the operation. | 6 |
| `NOT_IMPLEMENTED` | The command exists but its behavior has not landed. | 1 |
| `CONFIG` | Missing or invalid configuration. | 1 |
| `IO` | A local file, keyring, or stdout operation failed. | 1 |
| `INTERNAL` | An unexpected internal failure. | 1 |

Typed errors currently print `error: <message>` to stderr and exit 1. A JSON error shape is not emitted yet; when added, the code remains the contract and the message is redacted.

## Configuration and environment

Precedence, highest first: command-line flags; `SIMASTER_*` environment variables plus `RUST_LOG`, `NO_COLOR`, and `RUST_BACKTRACE`; the user config file in the platform config directory; built-in defaults. There is no project-local configuration, `.env` loading, or secret file; a checkout must never supply credentials. Invalid config is a `CONFIG` error, not a silent fallback.

| Variable | Status | Purpose |
|---|---|---|
| `SIMASTER_CLIENT_ID` | reserved | UGM ID public/native client ID; enables browser PKCE for `--method auto`. Not a secret. |
| `SIMASTER_CLIENT_SECRET` | confidential-client-only, experimental | Never used for a public client and never shipped. |
| `RUST_LOG` | available | Log filter; overrides `--verbose`. |
| `NO_COLOR` | available | Disable color in human output. |

Log precedence is `RUST_LOG` > `--verbose` > default `warn`. A public/native client uses `token_endpoint_auth_method=none`.

## Help, completions, and versioning

`--help` exits 0; `--version` prints `simaster <semver>` and exits 0. Current help is the clap default: summary, flags, and subcommands. When a command lands, its help adds examples, related commands, environment variables, and exit codes; help text is not a stable machine interface.

`simaster completions bash|zsh|fish|powershell|elvish` generates from the live command tree and is never hand-edited. The project follows Semantic Versioning; before 1.0, minor releases may contain breaking changes. `schema_version` versions independently of the crate: an optional field is non-breaking, while removing a field, changing a type, or changing an exit code is breaking. Experimental commands are marked `experimental` here and in `--help`.

## Agent contract

- Pass `--json` where supported and parse stdout; diagnostics go to stderr.
- Use the exit code and error code, not message text, for control flow.
- Do not assume a prompt; avoid commands that can prompt. Browser login needs a human for CAS/SSO and consent.
- Do not request, print, store, or transmit passwords, cookies, tokens, authorization codes, or student data.
- The CLI is read-only. A future write requires `--allow-write`, supports `--dry-run`, and requires confirmation unless `--yes` is given.
- Do not retry `UPSTREAM_CHANGED` or `PERMISSION_DENIED` without changing the request.
