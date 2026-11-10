# ADR 0002: Web session default, OAuth opt-in

- **Status:** Accepted
- **Date:** 2026-11-10
- **Deciders:** project maintainers

## Context

> **Evidence:** all upstream behavior in this ADR is `unverified`. The evidence
> is prior-art code and static analysis, not a live test. See
> [`../AUTH.md`](../AUTH.md) and [`../API-MAP.md`](../API-MAP.md) for the
> endpoint-level provenance and stability markers.

SIMASTER has three relevant authentication surfaces, as documented in prior
art and static analysis:

1. A web session login endpoint on `simaster.ugm.ac.id`, with a CAS fallback
   through `sso.ugm.ac.id` (`unverified`).
2. A vNext OAuth service used by the official mobile app (`unverified`).
3. No documented, public OAuth client for third-party CLI use is known to the
   project (`unverified`; absence of evidence is not proof).

Static analysis of the official app indicates that its OAuth flow embeds a
client secret in the binary, uses a custom-scheme redirect, and does not use
PKCE or `state` (`unverified`). Copying that client would distribute a
credential we do not own, bind the CLI to the app's identity, and reproduce
weaknesses that the OAuth specification warns against. Shipping the secret is
not an option.

A browser SSO flow with PKCE and a loopback redirect is the correct long-term
native-app model, but it requires a client registration and a redirect policy
that the project does not control. It is blocked on UGM; no live test exists.

Prior-art evidence indicates that a web session flow can work for a student
with their own account (`unverified`), but it is brittle: CAPTCHA, MFA, cookie
expiry, HTML parsing, and WAF behavior can change. It is still the most
realistic default for M2.

## Options

| Option | Security | Evidence and availability | Maintenance | Decision |
|---|---|---|---|---|
| Web session only | Medium: password in the CLI, cookie storage. | Prior-art `unverified` evidence; a live test with an authorized account is still required. | High: scraping and HTML changes. | Default, with CAS fallback. |
| CAS fallback only | Medium: same credentials, SSO form. | Prior-art `unverified` evidence; a live test is still required. | High: HTML and redirect changes. | Fallback, not the only path. |
| Official app OAuth with embedded secret | Low: secret is extractable; no PKCE/state; custom scheme. | Static `unverified` evidence; usable only by impersonating the app. | Low code, high legal and ethical risk. | Rejected. |
| vNext OAuth with user-supplied client | Medium-high: depends on the client; state + PKCE added. | Specified; `unverified` until a live test with an authorized client. | Medium. | Experimental, opt-in. |
| Browser SSO + PKCE | High: no password in the CLI, MFA-friendly, state + PKCE. | Blocked on client registration; `unverified`. | Low once registered. | Target for v1.0. |
| Cookie import | Medium: bearer credential, expiry, storage risk. | Candidate; `unverified` and not scheduled. | Medium. | Candidate, not scheduled. |

## Decision

- **Default:** web session login with a CAS fallback, based on `unverified`
  prior-art evidence and subject to live verification in M2.
- **Experimental:** vNext OAuth through `--method vnext`, using a user-supplied
  client from the environment or keyring. No embedded client ID or secret.
- **Target:** browser SSO with PKCE, `state`, and a loopback redirect for v1.0.
- **Rejected:** copying the official app's client credentials, custom-scheme
  interception, embedded secrets, and any flow without `state`/PKCE.
- **Non-goals:** CAPTCHA-solving services, MFA bypass, rate-limit evasion, and
  multi-user token reuse.

Credential storage is keyring-first with a `0600` file fallback. Passwords are
read from a prompt or stdin, never argv. Tokens and cookies are never logged.

## Consequences

Positive:

- The project can make progress without a UGM partnership.
- No third-party client credential is distributed.
- The experimental provider gives users with an authorized client a path
  forward without making OAuth the default.
- The v1.0 target is the correct native-app design.

Negative:

- The default flow handles passwords and cookies, which is a real security
  burden.
- The web flow can break without notice.
- vNext OAuth is not usable by most users until a client exists.
- Browser SSO is blocked, so the best option is not available yet.

Mitigations:

- Typed `AUTH_FAILED`, `AUTH_EXPIRED`, `RATE_LIMITED`, and `UPSTREAM_CHANGED`
  errors with actionable messages.
- Interactive CAPTCHA only; no automated solving.
- Keyring-first storage and a warning when the fallback is active.
- Sanitized fixtures and a scheduled read-only canary.
- A documented browser fallback when the CLI cannot complete authentication.

## Revisit triggers

Review this ADR when:

- UGM registers a CLI or native-app OAuth client, or publishes a supported API;
- UGM makes MFA mandatory for all students;
- the web session or CAS flow breaks and cannot be repaired without a browser;
- the OAuth server requires PKCE, changes the redirect policy, or issues
  different token lifetimes;
- a security review finds the credential handling below the standard in
  `SECURITY.md`;
- a user-supplied client becomes impractical for the target audience.

A change to the default method, storage model, or scope policy is an RFC under
`GOVERNANCE.md`.
