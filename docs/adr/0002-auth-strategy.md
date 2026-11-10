# ADR 0002: UGM ID PKCE primary, web session fallback

- **Status:** Accepted
- **Date:** 2026-11-10
- **Deciders:** project maintainers

## Context

UGM publishes an official UGM ID OAuth 2.0/OIDC service at `oauth.simaster.ugm.ac.id`. Its OpenAPI 3.1 specification, OIDC discovery, Swagger UI, and portal Developer section document Authorization Code with PKCE (`S256`), several token grants, client authentication methods `client_secret_basic`, `client_secret_post`, and `none`, resource endpoints `/api/me`, `/user/me`, `/userinfo`, and `/user/username/{username}`, scopes `openid profile email user.read userDetail.read`, and manual client registration through DTI UGM.

The authorize description says `code_challenge` is required, while the machine schema marks it optional and lists `plain`; discovery lists only `S256`. The discovery `issuer` and endpoints omit the scheme. The CLI treats PKCE as required, uses `S256` only, and fails closed on the literal issuer. The web session + CAS flow remains useful because browser PKCE cannot complete until DTI registers a client. The vNext gateway is a data adapter; its acceptance of an official UGM ID token is `unverified`. The mobile app embeds a client secret and uses a custom scheme; that approach is secondary evidence and is not used.

## Options

| Option | Security | Availability | Maintenance | Decision |
|---|---|---|---|---|
| UGM ID browser PKCE | High: public client, S256, `state`, `nonce`, loopback, ID-token validation. | Documented by UGM; registration pending. | Medium: discovery, JWKS, token lifecycle. | Primary. |
| Web session + CAS | Medium: password handling, cookie storage, HTML parsing. | Prior art, `unverified`; available without a client. | High: CAPTCHA, MFA, HTML, WAF changes. | Fallback. |
| vNext as auth provider | Low: needs the mobile secret or a custom client. | Rejected. | High legal and ethical risk. | Not an auth path. |
| vNext as data adapter | Medium: Bearer token; acceptance `unverified`. | Prior-art static evidence. | Medium: envelope and label drift. | Data adapter. |

## Decision

- **Primary:** browser Authorization Code + PKCE S256 with a public/native UGM ID client, loopback redirect `http://127.0.0.1:53682/callback`, `state`, `nonce`, and ID-token validation. Use it when `SIMASTER_CLIENT_ID` is configured.
- **Fallback:** SIMASTER web session with CAS until DTI registers a client or when the browser flow is unavailable.
- **vNext:** data adapter only. If it does not accept the official token, use the web data adapter. Never add a mobile-client auth provider or secret.
- **No write scopes by default.** Request only the read-only scopes needed.
- **Confidential-only fallback:** do not ship a secret. Keep the web fallback, allow a user-registered confidential client from the environment or keyring, or revisit a backend-for-frontend in an RFC. No BFF exists in v0.
- **Storage:** keyring-first, `0600` file fallback, no password storage for browser PKCE, redaction everywhere.

The current design is in [`docs/AUTH.md`](../AUTH.md).

## Consequences

Positive: the official identity API replaces reverse-engineered mobile auth; no third-party client credential is distributed; browser PKCE avoids password storage; the web fallback keeps the CLI useful while registration is pending; vNext can reach data UGM ID does not expose.

Negative: browser PKCE is blocked until DTI registers a client; `none` is legacy-only per the spec; the issuer/scheme inconsistency needs strict handling; a public client cannot introspect or revoke; the web fallback handles passwords and brittle HTML; vNext token acceptance is unresolved.

Mitigations: the DTI request in [#34](https://github.com/pendopo-dev/simaster/issues/34), the web fallback, mocked discovery/token/JWKS/user tests, a typed `UPSTREAM_CHANGED` path, and a documented public-client logout limitation.

## Revisit triggers

Review when DTI approves a public client or requires a confidential client; MFA becomes mandatory in a way that affects the web fallback; vNext accepts or rejects the official token; UGM changes the issuer, PKCE metadata, token lifetimes, refresh rotation, or rate limits; the web/CAS fallback breaks; or a security review finds a gap in PKCE, `state`, `nonce`, loopback, ID-token validation, or storage.

A change to the default method, storage model, scope policy, or client type is an RFC under [`GOVERNANCE.md`](../../GOVERNANCE.md).
