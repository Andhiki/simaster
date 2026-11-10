# Authentication

This document covers the authentication decision, official endpoints, PKCE flow, web fallback, storage, and security rules. Authentication and API adapters are not implemented yet; this is the design and evidence record. **Evidence status:** official UGM ID behavior is **documented by UGM** and **not live-tested** except the public `/health` check; vNext and web-session behavior is prior-art/static evidence and `unverified`.

## Decision

1. **Primary:** browser OAuth 2.0 Authorization Code + PKCE S256 with a
   public/native client and a loopback redirect, used when `SIMASTER_CLIENT_ID`
   is configured.
2. **Fallback:** SIMASTER web session with a CAS fallback while no UGM ID client
   is registered or when the browser flow is unavailable.
3. **vNext:** a **data adapter only**, never an authentication path. It may accept the official token; that is `unverified`. If not, use the web adapter.
4. **Rejected:** the mobile app's embedded secret, its custom scheme as a credential, the password grant, and any write scope by default.

The rationale is in [ADR 0002](adr/0002-auth-strategy.md).

## Official endpoints

Official links: [Swagger UI](https://oauth.simaster.ugm.ac.id/docs),
[OpenAPI 3.1](https://oauth.simaster.ugm.ac.id/openapi.json),
[OIDC discovery](https://oauth.simaster.ugm.ac.id/.well-known/openid-configuration),
[JWKS](https://oauth.simaster.ugm.ac.id/.well-known/jwks.json),
[health](https://oauth.simaster.ugm.ac.id/health).

| Method | Path | Scope | Purpose |
|---|---|---|---|
| `GET` | `/oauth/authorize` | requested | Start Authorization Code + PKCE; redirects through CAS SSO and consent. |
| `POST` | `/oauth/token` | requested | Exchange the code or refresh a token. |
| `GET` | `/userinfo` | `openid` | OIDC claims: `sub`, name, email, picture, preferred username. |
| `GET` | `/api/me` | `user.read` | Basic profile: username, email, name, picture, provider. |
| `GET` | `/user/me` | `userDetail.read` | Full Simaster profile: identity, type, attributes, blacklist flag. |
| `GET` | `/oauth/logout` | — | RP-initiated logout; optional `id_token_hint`. |
| `GET` | `/.well-known/openid-configuration` | — | OIDC discovery metadata. |
| `GET` | `/.well-known/jwks.json` | — | RS256 public keys for JWT verification. |
| `GET` | `/health` | — | Public health JSON; diagnostic only. |

Resource calls use `Authorization: Bearer <access_token>`. An unauthenticated `/api/me` returns HTML 401, not the OpenAPI JSON shape; map a non-JSON 401/403 to `AUTH_REQUIRED` or `AUTH_EXPIRED`, never a parse error. `/user/username/{username}` is out of scope (enumeration risk); `/oauth/introspect` and `/oauth/revoke` need HTTP Basic, so a public client cannot use them.

Scopes are `openid`, `profile`, `email`, `user.read` (`/api/me`), and `userDetail.read` (`/user/me`). Request `openid user.read` for identity, add `userDetail.read` for the full profile, and never request a write scope. The token response's `scope` is authoritative.

## Browser PKCE flow

```mermaid
sequenceDiagram
  autonumber
  actor U as Student
  participant CLI as simaster
  participant B as System browser
  participant ID as UGM ID
  participant CAS as CAS SSO
  participant API as UGM ID API

  U->>CLI: simaster auth login
  CLI->>CLI: generate state, nonce, verifier\nS256 challenge
  CLI->>CLI: start one-shot listener\n127.0.0.1:53682/callback
  CLI->>B: open /oauth/authorize\n(client_id, redirect_uri, scope,\nstate, nonce, code_challenge)
  B->>ID: GET /oauth/authorize
  ID->>CAS: redirect for SSO
  CAS-->>ID: authenticated session
  ID-->>B: consent screen
  U->>B: approve read-only scopes
  ID-->>CLI: 302 to loopback with code + state
  CLI->>CLI: verify state; close listener
  CLI->>ID: POST /oauth/token\n(code, redirect_uri, client_id, code_verifier)
  ID-->>CLI: access_token, optional refresh_token,\nid_token, expires_in, scope
  CLI->>CLI: validate ID token via JWKS and nonce\nstore tokens in keyring
  CLI->>API: Bearer access_token
```

- **PKCE is required.** Use `S256` only, never `plain`; the description says it is required even though the schema marks it optional and discovery lists only `S256`.
- **`state` and `nonce` are one-time.** Generate both with a secure random source, compare `state` exactly, and validate `nonce` against the ID token.
- **Loopback only.** Bind `127.0.0.1:53682`, accept one callback, close immediately, time out after 300 seconds, and never bind `0.0.0.0` or reuse the socket. If the browser cannot open, print the URL to stderr and keep waiting.
- **No secret for a public client.** Send `client_id` and `code_verifier`; a confidential client, if DTI requires one, reads its secret from the environment or keyring only.
- **Redaction.** Never print or log the authorization code, verifier, token, or callback URL; map `error=access_denied` to `AUTH_FAILED`.

The token endpoint also supports `password`, `client_credentials`, Google ID-token exchange, and RFC 8693 exchange. This CLI uses only `authorization_code` and `refresh_token`.

## Web session and CAS fallback

The fallback is used when `SIMASTER_CLIENT_ID` is absent and `--method auto` selects web, when the user selects `--method web` or `sso`, or after a browser failure when the user explicitly re-runs. There is no automatic downgrade mid-login.

- `POST /services/simaster/service_login` with `aId`, `username`, and `password` returns `isLogin` JSON and a session cookie.
- When direct login fails, `/ugmfw/signin_simaster/signin_proses` redirects to CAS; the CLI posts the login form and follows the ticket. If the portal presents a CAPTCHA or MFA challenge, the CLI stops and asks the user to complete it; it never solves or bypasses the challenge.
- The flow is prior-art and `unverified`; it involves password handling, brittle HTML, CAPTCHA and MFA challenges, and WAF changes, so it is lower assurance than browser PKCE.
- Delete the cookie through the session layer on logout. The portal may still consider the server-side session active; report that limitation.

## Method selection

| Method | Behavior |
|---|---|
| `auto` | Browser PKCE when `SIMASTER_CLIENT_ID` is configured; otherwise web session with CAS. |
| `browser` | Browser PKCE. Without a client ID, fail with `CONFIG`. |
| `web` | Direct SIMASTER web session only. |
| `sso` | CAS fallback only. |

`auto` never silently downgrades mid-login. If the browser flow fails, report
the error; the user can re-run with `--method web`.

## Storage

| Item | Storage | Removal |
|---|---|---|
| UGM ID access, refresh, and ID tokens | OS keyring first; `0600` file fallback. | Refresh, logout, or `AUTH_EXPIRED`. |
| Web session cookie | OS keyring first; `0600` fallback. | `auth logout`. |
| UGM password | Never stored. | — |

The file fallback uses `0600` on Unix or a restricted ACL on Windows, holds only the minimum fields, and never lives in the repository or a project directory. Intended behavior: `doctor` reports the backend and warns when the fallback is active; `auth status` reports provider, username, scopes, expiry, and storage, never a token value.

## Refresh, expiry, and logout

- Exchange a refresh token with `grant_type=refresh_token`. Until DTI documents rotation, do not reuse an old token when the response omits a new one; rejected refresh means `AUTH_EXPIRED`.
- Treat `expires_in` as authoritative; do not hard-code the documented 3600.
- Logout deletes local state first. A public client cannot introspect or revoke; open `/oauth/logout` with `id_token_hint` when available. Report partial failures and never claim server-side logout.

## Failure modes

| Failure | Detection | Result |
|---|---|---|
| Missing client ID | `SIMASTER_CLIENT_ID` absent with `browser` or browser `auto`. | `CONFIG`, exit 1; offer `--method web`. |
| Redirect mismatch | Authorization error, or no callback before timeout. | `AUTH_FAILED` with the registered URI, redacted. |
| Consent denied | `error=access_denied`. | `AUTH_FAILED`; no token stored. |
| CAS/MFA challenge | SSO asks for an additional factor. | Browser can complete; web returns `AUTH_FAILED` with a browser hint. |
| Rate limit | 429 or `Retry-After`. | `RATE_LIMITED`, exit 4; honor `Retry-After`. |
| Expired access token | 401 or passed `exp`. | `AUTH_EXPIRED`, exit 3; refresh once, then re-login. |
| HTML 401 | Non-JSON body from a resource call. | `AUTH_REQUIRED` or `AUTH_EXPIRED`, not a parse error. |
| Discovery scheme quirk | `iss` does not match the literal issuer. | Fail closed as `UPSTREAM_CHANGED`. |
| Upstream change | Missing field or unexpected status/page. | `UPSTREAM_CHANGED`, exit 5; update the fixture, do not guess. |

## Threat model

| Asset | Adversary | Mitigation |
|---|---|---|
| UGM password | Local malware, shoulder surfing, shell history. | PKCE avoids it; web fallback uses prompt or stdin, never argv. |
| Authorization code | Local app, redirect interception. | PKCE S256, one-time `state`, one-shot loopback, no code logging. |
| ID token | Substitution, replay, wrong audience. | RS256 via JWKS, `iss`, `aud`, `exp`, `nonce`, `kid`, algorithm allowlist. |
| Access/refresh token | File reader, log collector, crash report. | Keyring first, `0600` fallback, redaction, no token in URL. |
| Session cookie | File reader, shared terminal. | Keyring first, no logs, logout; web is lower assurance. |
| Student data | Another user, malicious agent. | Own account, read-only scopes, no reuse, no caching by default. |
| Build/release | Supply-chain attacker. | Locked dependencies, cargo-deny, Dependabot, signed tags, reviewed workflows. |

The CLI does not defend against a compromised operating system or a user who
shares their own credentials. It does defend against disclosure in its output,
logs, and storage.

## Secret policy and responsible use

- The mobile app's embedded secret and client ID are never copied, bundled, committed, logged, or documented as values; its custom-scheme redirect is not used.
- A public/native UGM ID client has no secret. `SIMASTER_CLIENT_SECRET` exists only for a confidential client and is never shipped.
- If DTI requires a confidential client, keep the web fallback, allow a user-registered client from the environment or keyring, or revisit a backend-for-frontend in an RFC. No BFF exists in v0.
- Use only your own account and data. Do not share, pool, or reuse tokens, and do not access another person's records, including through `/user/username`.
- Do not bypass CAPTCHA, MFA, rate limits, or authorization, and do not request write scopes. Follow UGM policy and applicable law.

Client registration is manual; discovery has no `registration_endpoint`. The DTI request, requested profile, and open questions are tracked in [#34](https://github.com/pendopo-dev/simaster/issues/34).
