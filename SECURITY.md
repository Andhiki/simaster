# Security policy

`simaster` handles credentials, session cookies, OAuth tokens, and student data.
Reports are welcome and handled privately.

## Supported versions

No release has been published yet. Unreleased `main` receives best-effort review with no support guarantee. Include the tested version or commit.

## Reporting a vulnerability

**Do not open a public issue, discussion, or pull request for a vulnerability.** Use [GitHub Security Advisories](https://github.com/pendopo-dev/simaster/security/advisories/new) with the affected version or commit, command, flag, platform, a minimal reproduction with secrets and personal data removed, the observed impact, and whether you can test a fix. If advisories are unavailable, contact a maintainer privately through the address on their GitHub profile. Do not include credentials in the first message.

## Response targets

Best-effort, not a contractual SLA: acknowledgement within 3 business days; assessment within 7 calendar days; fixes within 30 days for critical, 60 for high, and 90 for medium and low; an advisory within 90 days or after the fix ships. If a fix takes longer, the security maintainer explains the reason and the interim mitigation.

## Scope

| In scope | Out of scope |
|---|---|
| Secret leakage in output, logs, errors, fixtures, or docs. | UGM servers, SIMASTER, SSO, OAuth, or the mobile app. Report those to UGM. |
| Auth bypass, session fixation, PKCE downgrade, `state`/`nonce` mismatch, loopback exposure, ID-token bypass, unsafe refresh. | Social engineering, phishing, or physical access. |
| Insecure storage, a plaintext fallback without warning, insecure defaults, or missing write safeguards. | Denial of service, flooding, brute force, or missing headers on a host we do not control. |
| IDOR or cross-account access in this CLI's code or fixtures. | Reports requiring another person's account, data, or credentials. |
| Reachable dependency or supply-chain issues. | CAPTCHA solving, MFA bypass, or rate-limit evasion. |
| Documentation that could expose credentials or personal data. | |

This project cannot authorize testing of UGM systems.

## Safe harbor

We will not pursue legal action or report a researcher for accidental, good-faith research that follows these rules: test only with your own account and data; do not access, modify, or retain another person's data; do not run denial-of-service, load, or brute-force tests; do not test UGM production beyond normal use of your own account; stop immediately on encountering another person's data and delete it; and give maintainers a reasonable chance to fix the issue before disclosure. This safe harbor covers this project's code only; UGM policy and the law still apply.

## Authorization and credential rules

- **Browser PKCE only for UGM ID.** Authorization Code + PKCE `S256`, never `plain` or the password grant.
- **One-time `state` and `nonce`.** Use a secure random source, compare `state` exactly, and validate `nonce` against the ID token.
- **Loopback isolation.** Bind `127.0.0.1` only, accept one request, close, and enforce a finite timeout. No custom scheme or wildcard host.
- **ID-token validation.** Verify RS256 with the published JWKS, `kid`, `iss`, `aud`, `exp`, and `nonce`. Compare `iss` to the literal discovery issuer and fail closed as `UPSTREAM_CHANGED`; the published issuer omits `https://`.
- **No public-client secret.** A public/native client uses `token_endpoint_auth_method=none`. `SIMASTER_CLIENT_SECRET` is confidential-client-only and experimental; it is never shipped.
- **Logout limitation.** A public client cannot introspect or revoke; both need HTTP Basic. Delete local state and open `/oauth/logout` with `id_token_hint` when available; report partial failures.
- **No passwords in argv, no secrets in logs.** Read passwords from a prompt or stdin. Never log passwords, cookies, tokens, authorization codes, PKCE verifiers, client secrets, or full callback bodies; redact errors.
- **Keyring first, no local secrets, no reuse.** Persistent credentials use the OS keyring when available; the fallback is `0600` on Unix or a restricted ACL on Windows and never lives in a project directory. A checkout has no credential file, and a session belongs to one person.

## Upstream risk

Official UGM ID behavior is `documented by UGM` and `live-tested: no` until an authorized test is recorded; vNext and web behavior is `unverified`. Validate the literal discovery issuer and normalize transport only. Contract tests mock discovery, token, JWKS, and user endpoints and never contact production; a scheduled canary may detect drift with a maintainer's own account and fails with `UPSTREAM_CHANGED`. The CLI is read-only; future writes require `--allow-write`, `--dry-run`, and confirmation unless `--yes` is given. Do not bypass CAPTCHA, MFA, rate limits, or authorization.

## Disclosure timeline

1. Report received privately.
2. Acknowledgement within 3 business days; triage and severity within 7 days.
3. Fix or mitigation by severity, with updates every 14 days for critical and
   high issues.
4. Advisory after the fix ships or after 90 days, whichever comes first, with
   credit unless the reporter asks to remain anonymous.

The security maintainer coordinates with UGM when an issue involves a UGM system and will not publish details that could enable abuse.

## Security maintainers

The role and assignment are in [`GOVERNANCE.md`](GOVERNANCE.md). Until one is confirmed, the maintainers in `CODEOWNERS` receive reports and escalate privately. A maintainer with a conflict of interest recuses themselves.
