# Security policy

`simaster` handles credentials, session cookies, OAuth tokens, and student data.
Security reports are welcome and are handled privately.

## Supported versions

The project is pre-1.0. Only the **latest published release** is supported.
Unreleased commits on `main` are development code and receive best-effort
review, but no support guarantee. If you report a vulnerability, include the
version or commit you tested.

## Reporting a vulnerability

**Do not open a public issue, discussion, or pull request for a vulnerability.**

Use GitHub Security Advisories:

<https://github.com/Andhiki/simaster/security/advisories/new>

Include:

- the affected version or commit;
- the command, flag, and platform;
- a minimal reproduction with secrets and personal data removed;
- the impact you observed or believe is possible;
- whether you are willing to test a fix.

If GitHub Security Advisories are unavailable, contact a maintainer privately
through the address listed on their GitHub profile. Do not include credentials
in the first message.

## Response targets

These are best-effort targets, not a contractual SLA:

| Stage | Target |
|---|---|
| Acknowledgement | 3 business days |
| Initial assessment and severity | 7 calendar days |
| Critical fix or mitigation | 30 calendar days |
| High-severity fix | 60 calendar days |
| Medium- and low-severity fix | 90 calendar days |
| Coordinated advisory | Within 90 days of the report, or after the fix ships |

If a fix takes longer, the security maintainer explains the reason and the
interim mitigation in the advisory. A reporter may request an earlier
disclosure; the maintainer will balance user safety, upstream coordination, and
the reporter's preference.

## In scope

- Credential, cookie, token, or secret leakage in output, logs, errors, crash
  reports, fixtures, or documentation.
- Authentication bypass, session fixation, PKCE downgrade, missing or
  mismatched `state`/`nonce`, loopback listener exposure, ID-token validation
  bypass, or unsafe token refresh.
- Insecure credential storage, including world-readable files or plaintext
  fallback without an explicit warning.
- Insecure defaults, such as write operations enabled without opt-in.
- Missing or bypassable write-action safeguards: confirmation, dry run,
  idempotency, or audit output.
- IDOR or cross-account data access in this CLI's own code or fixtures.
- Dependency vulnerabilities that are reachable through this project.
- Supply-chain issues in the build, release, or CI configuration.
- Documentation that could cause a user to expose credentials or personal data.

## Out of scope

- Vulnerabilities in UGM servers, the SIMASTER portal, SSO, OAuth, or the
  official mobile app. Report those to UGM through its own channels. This
  project cannot authorize testing of UGM systems.
- Social engineering, phishing, or physical access.
- Denial of service, traffic flooding, or brute force against any host.
- Missing security headers or configuration on a host we do not control.
- Reports that require another person's account, data, or credentials.
- Automated CAPTCHA solving, MFA bypass, or rate-limit evasion. These are
  non-goals, not features to harden.

## Safe harbor

We will not pursue legal action or report a researcher for accidental,
good-faith research that follows these rules:

- Test only with **your own account** and your own data.
- Do not access, modify, or retain another person's data.
- Do not run denial-of-service, load, or brute-force tests.
- Do not test UGM production systems beyond normal use of your own account.
- Stop immediately if you encounter another person's data, and delete it.
- Give the maintainers a reasonable opportunity to fix the issue before public
  disclosure.

This safe harbor is limited to this project's code and does not grant
permission to test UGM infrastructure. UGM's own policies and the law still
apply.

## UGM ID authentication rules

- **Browser PKCE only for UGM ID.** Use Authorization Code + PKCE `S256`; never
  `plain` and never the password grant.
- **One-time `state` and `nonce`.** Generate both with a cryptographically
  secure random source, compare `state` exactly on the callback, and compare
  `nonce` against the ID token.
- **Loopback isolation.** Bind the callback listener to `127.0.0.1` only, accept
  one request, close immediately, and enforce a finite timeout. Do not use a
  custom scheme or a wildcard host.
- **ID-token validation.** Verify the RS256 signature with the published JWKS,
  the `kid`, `iss`, `aud`, `exp`, and `nonce`. Compare `iss` to the literal
  discovery issuer; do not silently rewrite it. The published issuer omits
  `https://`, so a mismatch fails closed as `UPSTREAM_CHANGED`.
- **No public-client secret.** A public/native client uses
  `token_endpoint_auth_method=none` and has no secret. `SIMASTER_CLIENT_SECRET`
  is confidential-client-only and experimental; it is never shipped.
- **Public-client logout limitation.** `/oauth/introspect` and `/oauth/revoke`
  require HTTP Basic client authentication. A public client cannot call them.
  Delete local state and open `/oauth/logout` with `id_token_hint` when
  available; report partial failures.
- **No dynamic registration.** DTI registers the client and redirect URI
  manually. The CLI cannot bootstrap a client on its own.

## Credential handling rules

These rules apply to code, tests, documentation, and support requests:

- **No passwords in argv.** Passwords are read from an interactive prompt or
  stdin, never from a command-line argument, environment variable by default,
  or shell history. Browser PKCE never handles the UGM password.
- **No secrets in logs.** Passwords, session cookies, access tokens, refresh
  tokens, ID tokens, authorization codes, PKCE verifiers, client secrets, and
  full request or callback bodies are never logged at any level. Errors are
  redacted before display.
- **Keyring first.** Persistent credentials use the operating-system keyring
  when available. The file fallback is created with mode `0600` on Unix and the
  equivalent restricted ACL on Windows, and is never written to a project
  directory.
- **No project-local secrets.** A repository checkout must not contain a
  `.env`, cookie jar, session file, or config file with credentials. Tests use
  placeholders and fixtures only.
- **No multi-user reuse.** A session or token belongs to one person. The CLI
  does not pool, share, proxy, or replay credentials across users.
- **Redaction at the boundary.** Output writers redact known-sensitive fields
  before printing. Debug logging is opt-in and still redacted.
- **No tokens in URLs.** Tokens and codes are not placed in query strings that
  are logged by proxies, shells, or browser history. OAuth uses `state`,
  `nonce`, and PKCE; the CLI never prints the authorization code, callback URL,
  or token. If a browser fallback prints the authorization URL, it contains no
  token and is still treated as sensitive.

## Upstream-risk policy

UGM ID is an official, documented service, but it is still an upstream
dependency that can change. The vNext gateway and the web session remain
undocumented and unstable. The project treats all of them conservatively:

- Official UGM ID behavior is `documented by UGM` and `live-tested: no` until an
  authorized test is recorded. vNext and web behavior is `unverified` until a
  live, authorized test is recorded.
- The discovery issuer and endpoint URLs omit the scheme; validate the literal
  issuer and normalize transport only. This is a server metadata inconsistency,
  not a reason to weaken validation.
- New endpoints ship behind an experimental status and an explicit opt-in.
- Contract tests mock discovery, token, JWKS, and user endpoints; they never
  contact production.
- A scheduled canary may detect drift; it uses a maintainer's own account and
  fails with `UPSTREAM_CHANGED` rather than guessing.
- Write operations are disabled by default, require `--allow-write` when they
  exist, support `--dry-run`, and require confirmation unless `--yes` is given.
- The project does not bypass CAPTCHA, MFA, rate limits, or authorization.
- If UGM registers a supported client or publishes a supported API, the project
  prefers that surface over reverse-engineered behavior.
- A public client cannot introspect or revoke tokens; local logout and browser
  logout are documented as the available path.

## Dependency security

Dependabot alerts, secret scanning, and push protection are enabled by the
repository owner. CI runs `cargo-deny` for advisories, licenses, duplicate
versions, wildcards, and unknown sources. A weekly `cargo-audit` workflow opens
an issue when a RustSec advisory affects the dependency graph.

Do not disable a security check to make CI green. If a check is wrong, document
why and add the narrowest exception with an expiry review.

## Disclosure timeline

1. Report received privately.
2. Acknowledgement within 3 business days.
3. Triage and severity within 7 calendar days.
4. Fix or mitigation according to severity, with status updates at least every
   14 days for critical and high issues.
5. Advisory published after the fix ships, or after 90 days, whichever comes
   first, unless the reporter and maintainers agree to a different date.
6. Credit the reporter unless they ask to remain anonymous.

The security maintainer coordinates with UGM if the issue involves a UGM system
or requires upstream action. The project will not publish details that could
enable abuse of UGM infrastructure.

## Security maintainers

The security maintainer role and its current assignment are in
[`GOVERNANCE.md`](GOVERNANCE.md). The role is currently vacant; until it is
assigned, the maintainers listed in `CODEOWNERS` receive reports and escalate
privately to the group. A maintainer with a conflict of interest recuses
themselves from the report. If no maintainer is available, use the private
contact method on a maintainer's GitHub profile and do not publish details.
