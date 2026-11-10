# ADR 0002: UGM ID PKCE primary, web session fallback

- **Status:** Accepted (revised 2026-11-10 after official UGM ID API discovery)
- **Date:** 2026-11-10
- **Deciders:** project maintainers

## Context

UGM publishes an official UGM ID OAuth 2.0/OIDC service at
`oauth.simaster.ugm.ac.id`. Its OpenAPI 3.1 specification, OIDC discovery
document, Swagger UI, and portal Developer section document:

- Authorization Code with PKCE (`S256`) for browser/native clients.
- A token endpoint that supports `authorization_code`, `refresh_token`,
  `password`, `client_credentials`, Google ID-token exchange, and RFC 8693
  token exchange.
- Client authentication methods `client_secret_basic`, `client_secret_post`, and
  `none`; `none` is described as legacy and slated for removal.
- Resource endpoints `/api/me`, `/user/me`, `/userinfo`, and
  `/user/username/{username}`.
- Scopes `openid`, `profile`, `email`, `user.read`, and `userDetail.read`.
- Manual client registration through DTI UGM, with no dynamic registration
  endpoint.
- A public health check and a documented development server.

The authorize description says PKCE `code_challenge` is required, while the
machine schema marks it optional and lists `plain`; discovery lists only `S256`.
The discovery `issuer` and endpoint URLs are published without a scheme. These
inconsistencies are documented in [`UGM-ID-API.md`](../UGM-ID-API.md) and handled
by failing closed on the literal issuer in security checks.

The web session + CAS flow remains useful because browser PKCE cannot complete
until DTI registers a client and redirect URI. The vNext gateway remains a data
adapter; its token acceptance for an official UGM ID access token is
`unverified`.

The official mobile app's OAuth flow embeds a client secret and uses a custom
scheme. Static analysis is secondary evidence, and the app's client ID, secret,
and custom-scheme redirect are not used by this project.

## Options

| Option | Security | Evidence and availability | Maintenance | Decision |
|---|---|---|---|---|
| UGM ID browser PKCE | High: public client, S256, state, nonce, loopback, ID-token validation. | Documented by UGM; client registration pending; live-tested no. | Medium: OIDC discovery, JWKS, token lifecycle. | Primary, M2. |
| Web session + CAS fallback | Medium: password handling, cookie storage, HTML parsing. | Prior-art `unverified` evidence; available without a UGM ID client. | High: CAPTCHA, MFA, HTML, WAF changes. | Fallback, M2. |
| vNext gateway as an auth provider | Low: requires the mobile app's embedded secret or a custom client. | Rejected. | High legal and ethical risk. | Not an auth path. |
| vNext gateway as a data adapter | Medium: Bearer token; official-token acceptance `unverified`. | Prior-art static evidence; secondary. | Medium: envelope and service-label drift. | Data adapter, M3. |
| Cookie import | Medium: bearer cookie, expiry, storage risk. | Candidate; `unverified` and not scheduled. | Medium. | Not scheduled. |
| Official mobile client secret | Low: extractable, no PKCE/state in the app flow, impersonation. | Static `unverified` evidence; not ours to distribute. | Low code, high legal and ethical risk. | Rejected. |

## Decision

- **Primary:** browser Authorization Code + PKCE S256 with a public/native UGM ID
  client, loopback redirect `http://127.0.0.1:53682/callback`, `state`, `nonce`,
  and ID-token validation. Use it when `SIMASTER_CLIENT_ID` is configured.
- **Fallback:** SIMASTER web session with CAS until DTI registers a client or
  when the browser flow is unavailable.
- **vNext:** data adapter only. It may accept the official UGM ID access token;
  that is `unverified`. If not, use the web data adapter. Never add a
  mobile-client auth provider or secret.
- **Profile PoC:** `auth login` (browser PKCE) → `whoami` via `/api/me` →
  `profile show` via `/user/me`. Both move to M2.
- **No mobile secret:** the app's client ID, embedded secret, custom-scheme
  redirect as a usable credential, tokens, and cookies are not copied or
  bundled.
- **No write scopes by default:** the CLI requests only the read-only scopes it
  needs.
- **Confidential-only fallback:** if DTI cannot register a public client, do not
  ship a secret. Keep the web fallback, allow a user-registered confidential
  client from the environment/keyring, or revisit a backend-for-frontend in a
  future RFC. No BFF exists in v0.
- **Credential storage:** keyring-first, `0600` file fallback, no password
  storage for browser PKCE, redaction everywhere.

## Consequences

Positive:

- The project uses the official, documented identity API instead of
  reverse-engineering the mobile app's auth.
- No third-party client credential is distributed.
- Browser PKCE is the correct native-app pattern and avoids password storage.
- The web fallback keeps the CLI useful while registration is pending.
- The vNext data adapter can still reach data that UGM ID does not expose, but
  it is no longer an auth path.

Negative:

- Browser PKCE is blocked until DTI registers a client and redirect URI.
- `none` is legacy-only per the spec; a new public client may be refused.
- The discovery issuer/scheme inconsistency requires strict, fail-closed
  handling.
- A public client cannot introspect or revoke; logout is partly local.
- The web fallback still handles passwords and brittle HTML.
- The vNext token-acceptance question is unresolved.

Mitigations:

- A ready-to-send DTI request and a U0 upstream track.
- The web session + CAS fallback remains available.
- Mocked discovery/token/JWKS/user tests and a live authorized test in M2.
- A typed `UPSTREAM_CHANGED` path for metadata drift.
- A documented public-client logout limitation.

## Revisit triggers

Review this ADR when:

- DTI approves a public/native client with `none` and PKCE;
- DTI requires a confidential client, or a confidential-only decision changes
  the viable path;
- MFA becomes mandatory in a way that affects the web fallback;
- the vNext gateway accepts or rejects the official UGM ID access token;
- UGM fixes the discovery issuer/scheme inconsistency or changes the PKCE
  metadata, token lifetimes, refresh rotation, or rate limits;
- the web session or CAS fallback breaks;
- a security review finds a gap in PKCE, state, nonce, loopback, ID-token
  validation, or storage handling.

A change to the default method, storage model, scope policy, or client type is
an RFC under [`GOVERNANCE.md`](../../GOVERNANCE.md).
