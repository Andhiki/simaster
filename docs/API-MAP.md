# API map

This document records the upstream surfaces the CLI may use, their evidence,
and the stability policy. The official UGM ID OAuth 2.0/OIDC API is the primary
authentication and identity surface. The vNext gateway and the web session are
data surfaces; neither is an authentication path.

> **Evidence quality:** official UGM ID behavior is **documented by UGM** and is
> **not live-tested**, except the public unauthenticated `/health` check. The
> vNext behavior is prior-art/static evidence and remains `unverified`. The
> web-session login flow and the akademik/beranda pages are additionally
> backed by captured traffic (user-supplied HARs from 2026-11-11, recorded in
> issue #52); captured observations are still `unverified` as a contract —
> upstream behavior is not guaranteed. Static analysis describes client code,
> not server behavior.
> See [`UGM-ID-API.md`](UGM-ID-API.md) for the full official-API reference.

## Surface levels

| Level | Surface | Role |
|---|---|---|
| **L0** | UGM ID OAuth 2.0/OIDC and resource API (`oauth.simaster.ugm.ac.id`). | Primary authentication and identity; `/api/me`, `/user/me`, `/userinfo`. |
| **L1** | Not used today. | Reserved; no adapter exists. The official identity surface is L0, and the data surfaces are L2/L3. |
| **L2** | vNext gateway (`api.simaster.ugm.ac.id`). | Simaster data adapter. Uses a Bearer token; official UGM ID token acceptance is `unverified`. |
| **L3** | Web session (`simaster.ugm.ac.id`). | Simaster data fallback; cookie-based. |

## Status legend

| Status | Meaning |
|---|---|
| **available** | Implemented and covered by tests in this repository. No upstream API surface is implemented in M1. |
| **planned** | Accepted for a future milestone; not implemented. |
| **experimental** | Implemented or specified behind an explicit opt-in; may change without notice. |
| **unverified** | Documented from non-live evidence. No server contract is assumed. |
| **blocked** | Not implemented and not planned in the current scope, with a stated reason. |

## Surfaces

| Level | Surface | Auth | Data | Stability | Scope |
|---|---|---|---|---|---|
| L0 | UGM ID OAuth 2.0/OIDC | Browser PKCE S256; public client; Bearer access token. | Identity, ID token, basic and full profile. | Documented by UGM; live-tested no (except public `/health`). | Read-only; `openid profile email user.read userDetail.read`. |
| L2 | vNext gateway | Bearer UGM ID token; `UGMFWSERVICE: 1`. Token acceptance is `unverified`. | Simaster data by service label. | `unverified`; prior-art evidence. | Read-only if accepted; otherwise use L3. |
| L3 | Web session | Username/password; session cookie; CAS fallback. | Simaster data: profile, schedule, grades, attendance. | `unverified`; HAR evidence for the login flow and akademik/beranda pages. OpenAPI: [`simaster-web`](openapi/simaster-web/openapi.yaml) (Scalar: `docs/api/index.html`). | Read-only. |
| — | Staff gateway and legacy presensi | Staff Bearer or client-derived credentials. | Staff data and attendance writes. | `blocked; unverified`. | Out of scope. |

## Endpoint registry

The `owner` column lists the people accountable for evidence and review, not the
only people allowed to change the row.

### L0 — official UGM ID

| Level | Surface | Method | Path | Auth | Request | Response | Provenance | Stability | Live-tested | Owner |
|---|---|---|---|---|---|---|---|---|---|---|
| L0 | UGM ID OAuth | `GET` | `https://oauth.simaster.ugm.ac.id/oauth/authorize` | Public client + PKCE; CAS SSO and consent. | Query `response_type=code`, `client_id`, `redirect_uri`, `scope`, `state`, `nonce`, `code_challenge`, `code_challenge_method=S256`. | `302` to CAS or `redirect_uri` with `code`; `200` consent page; `400` error. | OpenAPI 3.1 and OIDC discovery, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID OAuth | `POST` | `https://oauth.simaster.ugm.ac.id/oauth/token` | `none` for public client; `client_secret_basic`/`client_secret_post` for confidential client. | Form `grant_type`, `code`, `redirect_uri`, `client_id`, `code_verifier`; `refresh_token` for refresh. | JSON `access_token`, `expires_in`, optional `refresh_token`, `id_token` with `openid`, `scope`, `token_type`. | OpenAPI 3.1 and OIDC discovery, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID resource | `GET` | `https://oauth.simaster.ugm.ac.id/userinfo` | Bearer; `openid`. | No body. | OIDC claims: `sub`, `name`, `given_name`, `family_name`, `picture`, `email`, `email_verified`, `preferred_username`. | OpenAPI 3.1, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID resource | `GET` | `https://oauth.simaster.ugm.ac.id/api/me` | Bearer; `user.read`. | No body. | Basic profile: `username`, `email`, `name`, `given_name`, `family_name`, `picture`, `provider`. | OpenAPI 3.1, documented by UGM; unauthenticated 401 is HTML, not JSON. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID resource | `GET` | `https://oauth.simaster.ugm.ac.id/user/me` | Bearer; `userDetail.read`. | No body. | Full Simaster profile: `user_id`, `username`, `nama_lengkap`, `email`, `user_tipe`, `user_identitas`, `user_identitas_nomor`, `attributes`, `is_blacklist`. | OpenAPI 3.1, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID resource | `GET` | `https://oauth.simaster.ugm.ac.id/user/username/{username}` | Bearer; `user.read`. | Path username. | Full profile of the requested username. | OpenAPI 3.1, documented by UGM. | `blocked` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID lifecycle | `POST` | `https://oauth.simaster.ugm.ac.id/oauth/introspect` | HTTP Basic (client secret). | Form `token`, `token_type_hint`. | RFC 7662 introspection JSON. | OpenAPI 3.1 and discovery, documented by UGM. | `blocked` for a public client | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID lifecycle | `POST` | `https://oauth.simaster.ugm.ac.id/oauth/revoke` | HTTP Basic (client secret). | Form `token`, `token_type_hint`. | `200` even when the token is not found; `401` on bad client auth. | OpenAPI 3.1 and discovery, documented by UGM. | `blocked` for a public client | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID lifecycle | `GET` | `https://oauth.simaster.ugm.ac.id/oauth/logout` | None; optional `id_token_hint`. | Query `id_token_hint`, `post_logout_redirect_uri`, `state`. | `200` logout page or `302` to the post-logout URI. | OpenAPI 3.1 and discovery, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID operations | `GET` | `https://oauth.simaster.ugm.ac.id/health` | None. | No body. | Public health JSON; observed shape differs from the OpenAPI schema. | OpenAPI 3.1; public endpoint observed. | `planned` (diagnostic) | Yes, public unauthenticated | `@Andhiki @vityasyyy` |
| L0 | UGM ID metadata | `GET` | `https://oauth.simaster.ugm.ac.id/.well-known/jwks.json` | None. | No body. | RS256 public keys; cached five minutes per the spec. | OpenAPI 3.1 and discovery, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID metadata | `GET` | `https://oauth.simaster.ugm.ac.id/.well-known/openid-configuration` | None. | No body. | OIDC discovery metadata. | OIDC discovery, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |
| L0 | UGM ID metadata | `GET` | `https://oauth.simaster.ugm.ac.id/.well-known/oauth-authorization-server` | None. | No body. | RFC 8414 metadata. | OpenAPI 3.1, documented by UGM. | `planned` | No | `@Andhiki @vityasyyy` |

The authorize request sends OIDC `nonce` even though the OpenAPI parameter list
does not show it; it is required for ID-token binding. The request also sends
`state` and PKCE S256. The machine schema marks `code_challenge` optional while
the description says it is required; this project treats it as required.

### L2 — vNext gateway data

| Level | Surface | Method | Path | Auth | Request | Response | Provenance | Stability | Live-tested | Owner |
|---|---|---|---|---|---|---|---|---|---|---|
| L2 | vNext gateway | `POST` | `https://api.simaster.ugm.ac.id/v1/student/micro/services` | Bearer UGM ID token (acceptance `unverified`) + `UGMFWSERVICE: 1`. | Multipart `json1` (raw JSON) + `json` (base64url); body `{micro, service, query, param}`. | JSON; some recovered callers read `res` and treat 400 as a branch. | Prior-art static analysis; secondary evidence. | `experimental; unverified` | No | `@Andhiki @vityasyyy` |
| L2 | vNext gateway | `POST` | `https://api.simaster.ugm.ac.id/v1/staff/micro/services` | Staff Bearer + `UGMFWSERVICE: 1`. | Same envelope; staff profile label. | JSON with `res`. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| L2 | Account lookup | `POST` | `https://api.simaster.ugm.ac.id/staff` | Bearer + `UGMFWSERVICE: 1`. | Same envelope; username lookup label. | JSON with `res`, `is_active`, `user_tipe_nomor`; callers expect 400. | Static analysis; no recovered UI caller. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| L2 | Staff attendance | `POST` | `https://api.simaster.ugm.ac.id/v1/staff/micro/doscanent` | Bearer; no `UGMFWSERVICE` observed. | Multipart with QR, GPS, and identity fields. | Success body is discarded by the client. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |

### L3 — web session data

| Level | Surface | Method | Path | Auth | Request | Response | Provenance | Stability | Live-tested | Owner |
|---|---|---|---|---|---|---|---|---|---|---|
| L3 | Web session | `POST` | `https://simaster.ugm.ac.id/services/simaster/service_login` | Username/password form; cookie jar. | Form fields `aId`, `username`, `password`. | JSON with `isLogin`, `namaLengkap`; session cookie. | Prior-art `simaster.py`; secondary evidence. OpenAPI `paths/service-login.yaml`. | `unverified; planned` | No | `@Andhiki @vityasyyy` |
| L3 | CAS | `GET` | `https://sso.ugm.ac.id/cas/login?service=...` | CAS session; `lt` token. | Query `service`; login form carries the `lt` token. | HTML login form. | Prior-art `simaster.py`, `oauth.py`; form GET not captured. OpenAPI `paths/cas-login.yaml` (GET). | `unverified; planned` | No | `@Andhiki @vityasyyy` |
| L3 | CAS | `POST` | `https://sso.ugm.ac.id/cas/login` | CAS login form; `lt` token. | Form `username`, `password`, `lt`, `_eventId=submit`, `submit=LOGIN`; optional `service`; `;jsessionid` path variant in prior art. | `302` to `signin_proses?ticket=ST-…`. | HAR 2026-11-11 (issue #52) for the plain path and field names; prior art for the variant. OpenAPI `paths/cas-login.yaml` (POST). | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/signin_proses` | Cookie jar. | Optional `ticket` query. | Without ticket: `302` to CAS. With ticket: `301`/`302` chain (http→https round-trip observed) to `captchasound_verification`. | HAR 2026-11-11 (issue #52) for the ticket variant; prior art for the no-ticket variant. OpenAPI `paths/signin-proses.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/captchasound_verification` | Cookie jar. | No body. | HTML CAPTCHA challenge page carrying `simasterUGM_token`. | HAR 2026-11-11 (issue #52). OpenAPI `paths/captchasound-verification.yaml` (GET). | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `POST` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/captchasound_verification` | Cookie jar; `simasterUGM_token`. | Form with `simasterUGM_token`, `captcha`. | `302` to `/beranda/beranda/` on success; challenge re-presented on failure. | HAR 2026-11-11 (issue #52) for the success path; prior art for failure. OpenAPI `paths/captchasound-verification.yaml` (POST). | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/captcha_sound/` | Cookie jar. | Optional `_` cache-buster. | Audio CAPTCHA asset. | HAR 2026-11-11 (issue #52). OpenAPI `paths/captcha-sound.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/ugmfw/signin_simaster/captcha_sound_play` | Cookie jar. | No body documented. | Audio CAPTCHA asset (playback variant). | HAR 2026-11-11 (issue #52). OpenAPI `paths/captcha-sound-play.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view` | Session cookie. | No body. | HTML dashboard shell. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_menu_favorit` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-menu-favorit.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_akun` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-akun.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_agenda` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-agenda.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_notifikasi` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-notifikasi.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_attribute` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-attribute.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_fitur` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-fitur.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_important` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial. | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-important.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/beranda/beranda/view_buku` | Session cookie; `X-Requested-With: XMLHttpRequest`. | No body. | HTML partial (~85 KB handbook). | HAR 2026-11-11 (issue #52). OpenAPI `paths/beranda-view-buku.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/data_mahasiswa/` | Session cookie. | No body. | `302` to `view`. | HAR 2026-11-11 (issue #52). OpenAPI `paths/data-mahasiswa-index.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/data_mahasiswa/view` | Session cookie. | No body. | HTML page (~64 KB); profile data. | HAR 2026-11-11 (issue #52). OpenAPI `paths/data-mahasiswa-view.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/data_mahasiswa/pddikti_krs` | Session cookie. | No body. | HTML page (~60 KB). | HAR 2026-11-11 (issue #52). OpenAPI `paths/data-mahasiswa-pddikti-krs.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/data_mahasiswa/pddikti_biodata` | Session cookie. | No body. | HTML fragment (~3.5 KB). | HAR 2026-11-11 (issue #52). OpenAPI `paths/data-mahasiswa-pddikti-biodata.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/data_mahasiswa/dokumen` | Session cookie. | No body. | HTML fragment (~0.6 KB). | HAR 2026-11-11 (issue #52). OpenAPI `paths/data-mahasiswa-dokumen.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_krs/` | Session cookie. | No body. | `302` to `view`. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-krs-index.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_krs/view` | Session cookie. | No body. | HTML page (~8.5 KB); study plan. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-krs-view.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_khs/` | Session cookie. | No body. | `302` to `view`. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-khs-index.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_khs/view` | Session cookie. | No body. | HTML page (~9 KB); transcript list. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-khs-view.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_khs/content/{token}` | Session cookie. | Path `token`: opaque encrypted id from the KHS page. | HTML document viewer (~6–13 KB). | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-khs-content.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_kehadiran/` | Session cookie. | No body. | `302` to `view`. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-kehadiran-index.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Web session | `GET` | `https://simaster.ugm.ac.id/akademik/mhs_kehadiran/view` | Session cookie. | No body. | HTML page (~7 KB); attendance. | HAR 2026-11-11 (issue #52). OpenAPI `paths/mhs-kehadiran-view.yaml`. | `unverified; planned` | Yes (HAR) | `@Andhiki @vityasyyy` |
| L3 | Legacy presensi | `POST` | `https://simaster.ugm.ac.id/presensivnext/api/signin` | Basic client credential; client-derived password. | Form `username`, `password`. | JSON with a presensi token. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |
| L3 | Legacy presensi | `POST` | `https://simaster.ugm.ac.id/presensivnext/api/send_peserta` | Presensi Bearer. | Multipart attendance fields. | Attendance result. | Static analysis. | `blocked; unverified` | No | `@Andhiki @vityasyyy` |

The legacy presensi flow derives a credential from a client credential and a
username; this project does not reproduce it. The staff gateway, `/staff`,
`doscanent`, and `/user/username/{username}` are intentionally excluded.

## vNext gateway envelope

Static analysis shows a two-field multipart body with a JSON header:

- `Authorization: Bearer <token>`
- `UGMFWSERVICE: 1`
- `Content-Type: application/json;charset=UTF-8` (observed in the client)
- `Charset: utf-8`
- `accept: application/json`
- `json1`: the raw JSON body
- `json`: a base64url variant of the same JSON

The base64url variant is described as standard Base64 with `+` replaced by
`.`, `/` replaced by `_`, and `=` replaced by `-`. This is unusual and
`unverified`; capture the final wire format before depending on it.

The logical body is:

```json
{
  "micro": "student",
  "service": "<service-label>",
  "query": [],
  "param": []
}
```

`query` and `param` are operation-specific. Their exact shapes are
`unverified`. Some recovered callers compare the HTTP status with `400` and then
read a `res` field. Do not generalize that behavior: it needs a live capture.

The client has two networking stacks. The V1 stack uses a validated access
token and sends `UGMFWSERVICE: 1`; the V2 stack reads the stored token and omits
`UGMFWSERVICE`. The order in which interceptors overwrite the `Authorization`
header is `unverified`. This CLI uses one adapter per surface and records the
final request in a fixture test.

**Token acceptance is the key open question.** The official UGM ID access token
may or may not be accepted by the vNext gateway; this is `unverified`. Test it
with an authorized account in M3. If it is not accepted, use the web adapter;
do not add a mobile-client auth provider or a secret.

## Service labels

The gateway addresses operations by a label in the request body, not by a
separate URL. The inventory below comes from static analysis. All 29 labels are
`unverified`; presence in the binary is not proof that a label is callable,
authorized, or stable.

| # | Micro | Label | Area | Endpoint |
|---:|---|---|---|---|
| 1 | student | `get_agenda_kehadiran_rekap_student` | Attendance recap | `/v1/student/micro/services` |
| 2 | student | `get_agenda_kehadiran_rinci_student` | Attendance detail | `/v1/student/micro/services` |
| 3 | student | `get_banner_student` | Banner | `/v1/student/micro/services` |
| 4 | student | `get_berita_list_student` | News | `/v1/student/micro/services` |
| 5 | student | `get_jadwal_anda_agenda_student` | Schedule agenda | `/v1/student/micro/services` |
| 6 | student | `get_jadwal_anda_date_student` | Schedule by date | `/v1/student/micro/services` |
| 7 | student | `get_jadwal_anda_now_student` | Current schedule | `/v1/student/micro/services` |
| 8 | student | `get_jadwal_anda_studi_student` | Study schedule | `/v1/student/micro/services` |
| 9 | student | `get_konfirmasi_qr_student` | QR confirmation | `/v1/student/micro/services` |
| 10 | student | `get_riwayat_keaktifan_mhs` | Activity history | `/v1/student/micro/services` |
| 11 | student | `get_riwayat_pembayaran_mhs` | Payment history | `/v1/student/micro/services` |
| 12 | student | `get_riwayat_permohonan_mhs` | Request history | `/v1/student/micro/services` |
| 13 | student | `get_semester_student` | Semester or period | `/v1/student/micro/services` |
| 14 | staff | `get_simaster_atribut_by_username` | Account lookup | `/staff` |
| 15 | staff | `get_staff_profil_by_staff_id` | Staff profile | `/v1/staff/micro/services` |
| 16 | student | `get_student_beasiswa_progres_ajuan_by_niu` | Scholarship progress | `/v1/student/micro/services` |
| 17 | student | `get_student_beasiswa_tawar_by_tahun` | Scholarship offers | `/v1/student/micro/services` |
| 18 | student | `get_student_jadwal_ujian_by_niu` | Exam schedule | `/v1/student/micro/services` |
| 19 | student | `get_student_krs_by_niu_semester` | KRS and study results | `/v1/student/micro/services` |
| 20 | student | `get_student_oss_detail_by_id` | OSS detail | `/v1/student/micro/services` |
| 21 | student | `get_student_oss_list_by_niu` | OSS list | `/v1/student/micro/services` |
| 22 | student | `get_student_pengumuman_by_niu` | Announcements | `/v1/student/micro/services` |
| 23 | student | `get_student_profil_by_niu` | Student profile | `/v1/student/micro/services` |
| 24 | student | `get_student_sks__semester_by_niu` | SKS per semester | `/v1/student/micro/services` |
| 25 | student | `get_studi_jadwal_periode` | Period schedule | `/v1/student/micro/services` |
| 26 | student | `get_studi_kehadiran_rekap` | Attendance recap | `/v1/student/micro/services` |
| 27 | student | `get_studi_kehadiran_rinci` | Attendance detail | `/v1/student/micro/services` |
| 28 | student | `get_studi_krs` | KRS | `/v1/student/micro/services` |
| 29 | student | `get_studi_max_sks` | SKS limit | `/v1/student/micro/services` |

Labels 14 and 15 are staff surfaces and are `blocked`. The rest are candidates
for read-only student features.

## Profile endpoints

**Primary:** `GET https://oauth.simaster.ugm.ac.id/user/me` with scope
`userDetail.read`. It returns the full Simaster profile for the authenticated
account. `whoami` uses `GET /api/me` with scope `user.read` for the basic
identity.

**Fallback:** the web session profile page. The exact HTML shape is not
documented and is `unverified`; parse only after a fixture and a contract test.

**Secondary:** the vNext label `get_student_profil_by_niu`, only if the official
UGM ID token is accepted by the gateway. The exact `param` shape is
**unverified**. The suffix suggests a NIU, but static evidence does not establish
whether the server accepts a NIU in `param`, derives it from the token, or
expects a different array nesting. The CLI will not take an arbitrary NIU from
the user for this call.

`GET /user/username/{username}` is out of scope. It returns a full profile for
an arbitrary username and creates an enumeration/privacy risk with no product
need.

## Scopes and redirect

Official UGM ID scopes:

| Scope | Grants | Use |
|---|---|---|
| `openid` | OIDC ID token and `/userinfo`. | Required for identity and ID-token validation. |
| `profile` | Name, given name, family name, picture, preferred username. | Identity display. |
| `email` | Email and `email_verified`. | Only when the email is needed. |
| `user.read` | `/api/me` and the out-of-scope username lookup. | `whoami`. |
| `userDetail.read` | `/user/me`. | `profile show`. |

Least privilege:

- `whoami`/`auth status`: `openid user.read`, plus `profile email` when the
  display needs those claims.
- `profile show`: add `userDetail.read`.
- Never request write scopes. The default registration request is
  `openid profile email user.read userDetail.read`; commands request the
  smallest subset they need.

Redirect:

- Default: `http://127.0.0.1:53682/callback`.
- Optional, if DTI allows: `http://localhost:53682/callback`.
- Bind the listener to `127.0.0.1` only. Do not use a custom scheme, a wildcard
  host, or an external interface.
- The mobile app's custom scheme and embedded client secret are **not used** by
  this CLI and are not reproduced in this repository.

Client authentication:

- Public/native client: `token_endpoint_auth_method=none`; no secret. The spec
  describes `none` as legacy and says it will be removed, so DTI must confirm
  that a new client can use it.
- Confidential client: `client_secret_basic` or `client_secret_post`; the secret
  comes from the environment or keyring and is never shipped.
- `introspect` and `revoke` require Basic auth, so a public client cannot use
  them. Logout uses the browser endpoint; local deletion is the reliable part.
- No dynamic client registration: discovery has no `registration_endpoint`.
  DTI registers the client and redirect URI manually.

Discovery:

- Issuer and endpoint URLs are published without `https://` (for example,
  `issuer: oauth.simaster.ugm.ac.id`). Normalize only the transport URL. Do not
  silently rewrite the issuer in ID-token validation; compare the literal value
  and fail closed on a mismatch.
- `code_challenge_methods_supported` is `S256` only. The authorize description
  says `code_challenge` is required, while the machine schema marks it optional
  and lists `plain`. Treat PKCE as required and use S256 only.
- `jwks_uri` is the JWKS endpoint above; ID tokens are RS256.
- No `registration_endpoint`.

## Stability policy

1. **Official endpoint:** documented by UGM; starts `planned`/`documented`, not
   `available`. A contract test and a sanitized fixture are required before code
   is merged.
2. **Mocked tests:** auth tests mock discovery, token, JWKS, and user endpoints.
   They never contact production. A live authorized test is a separate,
   recorded event with the user's own account.
3. **Experimental data adapter:** vNext starts `experimental; unverified`. Test
   whether the official token is accepted before promoting it. If not, use the
   web adapter.
4. **Canary:** a nightly, opt-in job uses a maintainer's own account, never a
   shared account, and performs read-only calls. It is not part of PR CI.
5. **Feature flags:** browser PKCE is selected by `SIMASTER_CLIENT_ID`; vNext is
   selected only by an explicit data-adapter flag/status. A broken surface can
   be disabled without a release.
6. **Drift:** a missing field, a changed status, an unexpected HTML page, or a
   metadata mismatch produces `UPSTREAM_CHANGED` (exit 5). Do not guess, silently
   retry with different parameters, or rewrite the issuer.
7. **Removal:** a removed endpoint is documented in the changelog and the
   command is disabled with a clear error. There is no silent compatibility
   shim for an undocumented upstream.

## How to add or verify an endpoint

1. **Official documentation first.** Use the UGM ID OpenAPI/Swagger, OIDC
   discovery, or portal as primary evidence. Prior art is secondary and must be
   marked as such.
2. Record the surface, level, method, path, auth, request shape, response shape,
   provenance, date, and live-tested status.
3. Sanitize the evidence: remove names, NIU numbers, cookies, tokens, device
   identifiers, GPS, and any value that could identify a person. Replace them
   with documented placeholders.
4. Add a fixture under `tests/fixtures/` and a mocked contract test. The test
   must not contact production.
5. Update this file and the command documentation.
6. Request review. Auth, write, and security-relevant endpoints need two
   approvals.
7. Mark the endpoint `unverified` or `planned` until the live test is complete.
   Do not promote it based on a single happy path; record at least one error
   shape.

Never include the official mobile app's embedded client secret, a real client
ID, a real token, a real cookie, the mobile custom-scheme redirect, or real
student data in an issue, fixture, test, or documentation change.

## Responsible use

This map exists to build an interoperable client for a student's own data. It
is not a penetration-testing guide. Do not use it to:

- access another person's records, including through `/user/username`;
- enumerate accounts or identifiers;
- test write endpoints against production;
- bypass CAPTCHA, MFA, rate limits, or authorization;
- run load, fuzz, or denial-of-service tests against UGM;
- share or pool sessions and tokens.

If you find an upstream vulnerability, report it to UGM/DTI through its own
channels. Do not publish exploit details or another person's data. The project's
coordinated disclosure policy is in [`SECURITY.md`](../SECURITY.md).
