# UGM ID API reference

UGM ID is the official OAuth 2.0 and OpenID Connect identity service for
Universitas Gadjah Mada. This document is the project's concise, actionable
reference for using it from a native CLI. It is original prose based on the
public UGM ID documentation; it is not a copy of the specification.

## Provenance and status

- **Documented by UGM:** Swagger UI, OpenAPI 3.1, OIDC discovery, and the
  Developer section of the UGM ID portal.
- **Live-tested:** no, except the public unauthenticated `/health` check. No
  authenticated request was made during this revision, and no credentials are
  available to this project.
- **Revision date:** 2026-11-10.
- **Primary evidence:** official UGM ID documentation listed below. Prior-art
  mobile-app analysis remains secondary evidence for the vNext **data** adapter,
  not for authentication.

Official links:

| Resource | URL |
|---|---|
| Swagger UI | <https://oauth.simaster.ugm.ac.id/docs> |
| OpenAPI 3.1 specification | <https://oauth.simaster.ugm.ac.id/openapi.json> |
| OIDC discovery | <https://oauth.simaster.ugm.ac.id/.well-known/openid-configuration> |
| JWKS | <https://oauth.simaster.ugm.ac.id/.well-known/jwks.json> |
| Health check | <https://oauth.simaster.ugm.ac.id/health> |
| Development server | <https://oauth.dev.ugm.ac.id> |

The OpenAPI document describes the production server as
`https://oauth.simaster.ugm.ac.id` and the development server as
`https://oauth.dev.ugm.ac.id`. No dynamic client registration endpoint exists;
clients are registered by the UGM Directorate of Information Technology (DTI).

## Endpoints

Status values in this table: `planned` means accepted for M2/M3 but not
implemented; `out of scope` means deliberately excluded; `blocked` means the
public-client model cannot use it as documented. The stability column in
[`API-MAP.md`](API-MAP.md) uses equivalent markers; `out of scope` here
corresponds to `blocked` there.

| Method | Path | Auth | Scope | Purpose | Status |
|---|---|---|---|---|---|
| `GET` | `/oauth/authorize` | Public client + PKCE | Requested scopes | Start Authorization Code + PKCE; redirects through CAS SSO and consent. | planned (M2) |
| `POST` | `/oauth/token` | `none` for a public client, or `client_secret_basic`/`client_secret_post` for a confidential client | Requested scopes | Exchange the code, refresh a token, or use another documented grant. | planned (M2) |
| `GET` | `/userinfo` | Bearer | `openid` | OIDC claims: `sub`, name, email, picture, preferred username. | planned (M2) |
| `GET` | `/api/me` | Bearer | `user.read` | Basic profile: username, email, name, picture, provider. | planned (M2) |
| `GET` | `/user/me` | Bearer | `userDetail.read` | Full Simaster profile: identity, type, attributes, blacklist flag. | planned (M2) |
| `GET` | `/user/username/{username}` | Bearer | `user.read` | Full profile by arbitrary username. | out of scope |
| `POST` | `/oauth/introspect` | HTTP Basic (client secret) | — | RFC 7662 token introspection. | blocked for a public client |
| `POST` | `/oauth/revoke` | HTTP Basic (client secret) | — | RFC 7009 token revocation. | blocked for a public client |
| `GET` | `/oauth/logout` | None; optional `id_token_hint` | — | RP-initiated OIDC logout. | planned (M2) |
| `GET` | `/health` | None | — | Public service health JSON. | planned (diagnostic) |
| `GET` | `/.well-known/jwks.json` | None | — | RS256 public keys for JWT verification. | planned (M2) |
| `GET` | `/.well-known/openid-configuration` | None | — | OIDC discovery metadata. | planned (M2) |
| `GET` | `/.well-known/oauth-authorization-server` | None | — | RFC 8414 authorization-server metadata. | planned (M2) |

The resource API requires `Authorization: Bearer <access_token>`. An
unauthenticated `/api/me` returns an **HTML 401 page**, not the JSON error shape
in the OpenAPI document. The CLI must treat any non-JSON 401/403 body as an auth
failure, not as a parse error: `AUTH_REQUIRED` when there is no usable session,
`AUTH_EXPIRED` when a stored token was rejected.

`/user/username/{username}` is **out of scope**. It accepts an arbitrary
username and returns a full profile; implementing it would create an
enumeration and cross-account privacy risk with no product need. The CLI uses
`/user/me` for the authenticated account.

## Scopes

| Scope | Grants | Least-privilege use |
|---|---|---|
| `openid` | OIDC ID token and `/userinfo` claims. | Required for OIDC identity and ID-token validation. |
| `profile` | Name, given name, family name, picture, preferred username. | Request when displaying the authenticated identity. |
| `email` | Email and `email_verified`. | Request only when the email is needed. |
| `user.read` | `/api/me` and `/user/username/{username}`. | Required for `whoami`; the username lookup is out of scope. |
| `userDetail.read` | `/user/me`. | Required for `profile show`. |

Least-privilege recommendation:

- `whoami` / `auth status`: `openid user.read` plus `profile email` only when
  the display needs those claims.
- `profile show`: add `userDetail.read`.
- Never request write scopes. UGM ID's documented scopes are read-only; the
  project does not add a write scope to the default request.

The client-registration request in this document asks for
`openid profile email user.read userDetail.read`. Commands should still request
the smallest set that makes the command work, and the token response's `scope`
field is the source of truth for what was granted.

## Token response and lifetimes

The token endpoint returns JSON with:

| Field | Meaning | Notes |
|---|---|---|
| `access_token` | JWT bearer token. | Send as `Authorization: Bearer <token>`. |
| `expires_in` | Access-token lifetime in seconds. | The documented example is `3600`; treat the actual value as authoritative. |
| `refresh_token` | JWT refresh token. | Optional; not every response includes one. |
| `id_token` | OIDC ID token. | Present when `openid` is requested. |
| `scope` | Granted scopes. | May be narrower than the request. |
| `token_type` | Token type. | Documented example: `Bearer`. |

Additional documented grants (`password`, `client_credentials`, Google ID-token
exchange, and RFC 8693 token exchange) are **not used** by this CLI. The CLI
uses `authorization_code` and, when a refresh token is present, `refresh_token`.

The exact refresh-token lifetime and rotation policy are not documented in the
evidence available to this project. They are questions for DTI; until answered,
treat the refresh token as optional and re-run the browser flow when it is
missing or rejected.

## Browser PKCE flow

This is the primary authentication path when `SIMASTER_CLIENT_ID` is configured.

```mermaid
sequenceDiagram
  autonumber
  actor U as Student
  participant CLI as simaster
  participant B as System browser
  participant ID as UGM ID
  participant CAS as CAS SSO
  participant API as UGM ID resource API

  U->>CLI: simaster auth login
  CLI->>CLI: generate state, nonce, verifier\nS256 challenge
  CLI->>CLI: start one-shot listener\n127.0.0.1:53682/callback
  CLI->>B: open /oauth/authorize\n(client_id, redirect_uri, scope,\nstate, nonce, code_challenge)
  B->>ID: GET /oauth/authorize
  ID->>CAS: redirect for SSO
  CAS-->>ID: authenticated session
  ID-->>B: consent screen
  U->>B: approve requested scopes
  ID-->>CLI: 302 to loopback with code + state
  CLI->>CLI: verify state; close listener
  CLI->>ID: POST /oauth/token\n(code, redirect_uri, client_id, code_verifier)
  ID-->>CLI: access_token, optional refresh_token,\nid_token, expires_in, scope
  CLI->>CLI: validate ID token via JWKS\nand nonce; store tokens in keyring
  CLI->>API: Bearer access_token
```

Requirements:

1. Generate `state`, `nonce`, and a PKCE verifier with a cryptographically secure
   random source. The verifier is 43–128 characters from the unreserved set;
   derive the challenge as `BASE64URL(SHA256(verifier))` without padding.
2. Send `response_type=code`, `client_id`, `redirect_uri`, `scope`, `state`,
   `nonce`, `code_challenge`, and `code_challenge_method=S256`.
3. Use `http://127.0.0.1:53682/callback` as the default redirect. Bind the
   listener to `127.0.0.1` only. Do not bind `0.0.0.0` or an external interface.
4. Accept exactly one callback and close the listener immediately. Ignore
   unexpected paths, methods, or repeated callbacks. Use a finite timeout
   (default: 300 seconds) and fail with `AUTH_FAILED` when it expires.
5. Verify `state` exactly before using the `code`. Reject a missing, duplicate,
   or mismatched state. Never log the code, verifier, token, or full callback
   URL.
6. Handle `error` and `error_description` from the authorization response
   (for example, `access_denied` when consent is refused) and map them to
   `AUTH_FAILED` with a redacted message.
7. Exchange the code with
   `grant_type=authorization_code`, `code`, `redirect_uri`, `client_id`, and
   `code_verifier`. A public client sends **no client secret**. If the client is
   confidential, the secret comes from the environment or keyring; it is never
   compiled in, committed, or printed.
8. If the browser cannot be opened, print the authorization URL to stderr and
   ask the user to open it manually. Keep waiting for the loopback callback
   until the timeout; do not fall back to a custom scheme.

The OpenAPI description says `code_challenge` is **required**, while its machine
schema marks it optional and lists `S256` and `plain`. OIDC discovery lists only
`S256`. This project treats PKCE as required and uses **S256 only**; it never
sends `plain`. The contradiction is recorded in the caveats.

## ID-token validation

The ID token is a JWT. Validate it locally before trusting any claim:

1. **Signature:** fetch `/.well-known/jwks.json`, select the key by `kid`, and
   verify the RS256 signature. Allow only `RS256`; reject `none` and algorithm
   confusion. Cache the JWKS for at most five minutes and refresh on an unknown
   `kid` once.
2. **Issuer:** compare `iss` to the literal `issuer` value from the discovery
   document. Do not silently rewrite the issuer in the security check. The
   published value is `oauth.simaster.ugm.ac.id` with no scheme, which is a
   metadata inconsistency; a scheme-qualified token issuer that does not match
   the literal value fails closed and is reported as `UPSTREAM_CHANGED`.
3. **Audience:** `aud` must contain the configured `client_id`. Accept the JWT
   string or array form.
4. **Expiry:** reject an expired token, allowing a small clock skew (60
   seconds). Reject an `iat` that is unreasonably far in the future.
5. **Nonce:** the `nonce` claim must exactly match the value generated for this
   login attempt. A missing or mismatched nonce fails the login.
6. **Subject:** `sub` must be present. Use it only as an identity correlation
   value, never as proof of authorization by itself.

The ID token is used to establish the authenticated identity and to supply
`id_token_hint` for logout. It is not sent to the resource API; resource calls
use the access token.

## Client registration

The UGM ID portal's Developer section says:

> Hubungi tim DTI UGM untuk mendaftarkan `client_id` dan `redirect_uri`
> aplikasi Anda.

Contact: **dti@ugm.ac.id** (from the portal's `mailto:` link).

Registration is manual. Discovery has no `registration_endpoint`, so dynamic
client registration is not available. The CLI cannot complete the browser PKCE
flow until DTI registers the client and the redirect URI.

### Requested client profile

Ask DTI for a **public/native client** with:

| Parameter | Requested value |
|---|---|
| Client type | Public/native application |
| `token_endpoint_auth_method` | `none` (if allowed for a new client) |
| Grant types | `authorization_code`, `refresh_token` |
| PKCE | Required, `S256` only |
| Redirect URIs | `http://127.0.0.1:53682/callback` and, if allowed, `http://localhost:53682/callback` |
| Scopes | `openid profile email user.read userDetail.read` |
| Development client | A separate client on `https://oauth.dev.ugm.ac.id` |

The default port `53682` is the CLI's contract. Do not request a custom scheme;
a loopback redirect is the native-app pattern that this CLI can actually receive.

### Questions for DTI

1. Is a public/native client with PKCE and `token_endpoint_auth_method=none`
   allowed for a new client?
2. Are loopback redirects (`127.0.0.1`) allowed? Is a fixed port acceptable, or
   does DTI require a dynamic port?
3. What rate limits apply to the authorization, token, and resource endpoints?
4. What are the access-token and refresh-token lifetimes? Is the refresh token
   rotated on use?
5. Is a UGM ID access token accepted by `api.simaster.ugm.ac.id` (the vNext
   gateway), or is a separate client or scope required?
6. Is `/user/username/{username}` self-only, or can any valid token read another
   user's profile?
7. Where should security findings about UGM ID be reported?

Question 6 does not change this project's decision: the CLI will not implement
`/user/username` even if it is self-only. The endpoint is unnecessary for
`whoami` and `profile show`.

### If DTI requires a confidential client

Do **not** ship a client secret in the CLI. Choose one of these paths:

1. Keep the web session + CAS fallback as the supported authentication path.
2. Let a user register and supply their own confidential client through the
   environment or keyring; the secret is never committed or bundled.
3. Revisit a backend-for-frontend or another officially supported grant in a
   future RFC. No backend-for-frontend exists in v0.

A public client with `none` is preferred. The specification describes `none` as
legacy and says it will be removed in the future, so DTI's answer is required
before relying on it.

### Ready-to-send email (English)

```text
Subject: Request for a public/native UGM ID OAuth client for simaster-cli

Hello DTI UGM,

I am a UGM student and maintainer of simaster-cli, an unofficial,
terminal-first client for SIMASTER. The project is open source and read-only.
We would like to use the official UGM ID OAuth 2.0/OIDC service.

Could you register a public/native client for the project with:

- token_endpoint_auth_method: none (if allowed for a new client)
- grant types: authorization_code and refresh_token
- PKCE: required, S256 only
- redirect URIs: http://127.0.0.1:53682/callback and, if allowed,
  http://localhost:53682/callback
- scopes: openid profile email user.read userDetail.read
- a development client on https://oauth.dev.ugm.ac.id

We are not requesting write scopes.

Questions:
1. Is a public/native client with PKCE and auth method "none" allowed for a new
   client?
2. Are loopback redirects (127.0.0.1) allowed? Fixed port or dynamic port?
3. What rate limits apply?
4. What are the access/refresh token lifetimes? Is the refresh token rotated?
5. Is a UGM ID access token accepted by api.simaster.ugm.ac.id (vNext), or is a
   separate client/scope required?
6. Is /user/username/{username} self-only?
7. Where should security findings be reported?

Project URL: <project-url>
Maintainer: <name> (<github-handle>)

Thank you,
<name>
```

### Ready-to-send email (Indonesian)

```text
Subjek: Permohonan client OAuth publik/native UGM ID untuk simaster-cli

Halo tim DTI UGM,

Saya mahasiswa UGM dan maintainer simaster-cli, klien terminal tidak resmi
untuk SIMASTER. Proyek ini open source dan hanya membaca data. Kami ingin
menggunakan layanan resmi UGM ID OAuth 2.0/OIDC.

Mohon bantuan untuk mendaftarkan client publik/native dengan:

- token_endpoint_auth_method: none (jika diizinkan untuk client baru)
- grant type: authorization_code dan refresh_token
- PKCE: wajib, hanya S256
- redirect URI: http://127.0.0.1:53682/callback dan, jika diizinkan,
  http://localhost:53682/callback
- scope: openid profile email user.read userDetail.read
- client development terpisah di https://oauth.dev.ugm.ac.id

Kami tidak meminta scope tulis.

Pertanyaan:
1. Apakah client publik/native dengan PKCE dan metode autentikasi "none"
   diizinkan untuk client baru?
2. Apakah redirect loopback (127.0.0.1) diizinkan? Port tetap atau dinamis?
3. Berapa rate limit yang berlaku?
4. Berapa masa berlaku access/refresh token? Apakah refresh token dirotasi?
5. Apakah access token UGM ID diterima oleh api.simaster.ugm.ac.id (vNext),
   atau perlu client/scope terpisah?
6. Apakah /user/username/{username} hanya dapat mengakses akun sendiri?
7. Ke mana temuan keamanan tentang UGM ID harus dilaporkan?

URL proyek: <project-url>
Maintainer: <nama> (<github-handle>)

Terima kasih,
<nama>
```

## Caveats

- **No dynamic registration.** Discovery has no `registration_endpoint`; every
  client and redirect URI is registered manually by DTI.
- **`none` is legacy-only per the specification.** The spec says it is for
  legacy clients and will be removed; ask DTI before depending on it.
- **Discovery URLs omit the scheme.** The discovery `issuer` and endpoint URLs
  are published without `https://`. Use the literal issuer in security checks;
  normalize only the transport URL. Do not silently rewrite the issuer.
- **Introspection and revocation require Basic auth.** A public client cannot
  call `/oauth/introspect` or `/oauth/revoke`. Local logout works; server-side
  revocation is a limitation to document to the user.
- **Unauthenticated resource calls return HTML 401.** The CLI maps non-JSON
  auth failures to `AUTH_REQUIRED` or `AUTH_EXPIRED`.
- **Rate limits are unknown.** Implement a bounded, identifiable User-Agent and
  honor `Retry-After`; do not retry aggressively.
- **Token lifetimes beyond `expires_in` are unknown.** `refresh_token` may be
  absent. Re-run the browser flow when refresh is unavailable.
- **Health schema mismatch.** The public `/health` response observed during this
  revision has string service values (`"up"`) and top-level keys, while the
  OpenAPI `HealthResponse` schema describes objects with `status`. Treat the
  health body as diagnostic, not a stable contract.
- **`/user/username/{username}` is out of scope.** It is unnecessary for the
  self-service CLI and would create an enumeration/privacy risk.
- **PKCE metadata is contradictory.** The authorize description says
  `code_challenge` is required; the machine schema marks it optional and lists
  `plain`. Discovery lists only `S256`. This project requires PKCE and uses
  `S256` only.
- **The OpenAPI authorize parameter list omits `nonce`.** OIDC uses `nonce` to
  bind the ID token to the login attempt. Send it, validate it, and fail closed
  if the server rejects it.

## Security and privacy commitments

- Use only the authenticated user's own account and data.
- Never store the UGM password. Browser PKCE does not see it.
- Store access, refresh, and ID tokens in the OS keyring first, with a `0600`
  file fallback only when the keyring is unavailable.
- Never log tokens, authorization codes, PKCE verifiers, cookies, or full
  callback URLs.
- Do not share, pool, or reuse tokens across users.
- Do not request write scopes. The CLI is read-only.
- Use an identifiable User-Agent (`simaster-cli/<version>`) and honor rate
  limits.
- Report UGM ID security issues to DTI or the contact DTI provides; report
  issues in this CLI through its own security policy.
